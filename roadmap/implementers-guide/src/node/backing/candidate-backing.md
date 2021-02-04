# Candidate Backing

The Candidate Backing subsystem ensures every parablock considered for relay block inclusion has been seconded by at least one validator, and approved by a quorum. Parablocks for which no validator will assert correctness are discarded. If the block later proves invalid, the initial backers are slashable; this gives tmi a rational threat model during subsequent stages.

Its role is to produce backable candidates for inclusion in new relay-chain blocks. It does so by issuing signed [`Statement`s][statement] and tracking received statements signed by other validators. Once enough statements are received, they can be combined into backing for specific candidates.

Note that though the candidate backing subsystem attempts to produce as many backable candidates as possible, it does _not_ attempt to choose a single authoritative one. The choice of which actually gets included is ultimately up to the block author, by whatever metrics it may use; those are opaque to this subsystem.

Once a sufficient quorum has agreed that a candidate is valid, this subsystem notifies the [Provisioner][pv], which in turn engages block production mechanisms to include the parablock.

## Protocol

Input: [`CandidateBackingMessage`][cbm]

Output:

- [`CandidateValidationMessage`][cvm]
- [`RuntimeApiMessage`][ram]
- [`CandidateSelectionMessage`][csm]
- [`ProvisionerMessage`][pm]
- [`PoVDistributionMessage`][pdm]
- [`StatementDistributionMessage`][sdm]

## Functionality

The [Candidate Selection][cs] subsystem is the primary source of non-overseer messages into this subsystem. That subsystem generates appropriate [`CandidateBackingMessage`s][cbm] and passes them to this subsystem.

This subsystem requests validation from the [Candidate Validation][cv] and generates an appropriate [`Statement`][statement]. All `Statement`s are then passed on to the [Statement Distribution][sd] subsystem to be gossiped to peers. When [Candidate Validation][cv] decides that a candidate is invalid, and it was recommended to us to second by our own [Candidate Selection][cs] subsystem, a message is sent to the [Candidate Selection][cs] subsystem with the candidate's hash so that the collator which recommended it can be penalized.

The subsystem should maintain a set of handles to Candidate Backing Jobs that are currently live, as well as the relay-parent to which they correspond.

### On Overseer Signal

- If the signal is an [`OverseerSignal`][overseersignal]`::ActiveLeavesUpdate`:
  - spawn a Candidate Backing Job for each `activated` head, storing a bidirectional channel with the Candidate Backing Job in the set of handles.
  - cease the Candidate Backing Job for each `deactivated` head, if any.
- If the signal is an [`OverseerSignal`][overseersignal]`::Conclude`: Forward conclude messages to all jobs, wait a small amount of time for them to join, and then exit.

### On Receiving `CandidateBackingMessage`

- If the message is a [`CandidateBackingMessage`][cbm]`::GetBackedCandidates`, get all backable candidates from the statement table and send them back.
- If the message is a [`CandidateBackingMessage`][cbm]`::Second`, sign and dispatch a `Seconded` statement only if we have not seconded any other candidate and have not signed a `Valid` statement for the requested candidate. Signing both a `Seconded` and `Valid` message is a double-voting misbehavior with a heavy penalty, and this could occur if another validator has seconded the same candidate and we've received their message before the internal seconding request. After successfully dispatching the `Seconded` statement we have to distribute the PoV.
- If the message is a [`CandidateBackingMessage`][cbm]`::Statement`, count the statement to the quorum. If the statement in the message is `Seconded` and it contains a candidate that belongs to our assignment, request the corresponding `PoV` from the `PoVDistribution` and launch validation. Issue our own `Valid` or `Invalid` statement as a result.

> big TODO: "contextual execution"
>
> - At the moment we only allow inclusion of _new_ parachain candidates validated by _current_ validators.
> - Allow inclusion of _old_ parachain candidates validated by _current_ validators.
> - Allow inclusion of _old_ parachain candidates validated by _old_ validators.
>
> This will probably blur the lines between jobs, will probably require inter-job communication and a short-term memory of recently backable, but not backed candidates.

## Candidate Backing Job

The Candidate Backing Job represents the work a node does for backing candidates with respect to a particular relay-parent.

The goal of a Candidate Backing Job is to produce as many backable candidates as possible. This is done via signed [`Statement`s][stmt] by validators. If a candidate receives a majority of supporting Statements from the Parachain Validators currently assigned, then that candidate is considered backable.

### On Startup

- Fetch current validator set, validator -> parachain assignments from [`Runtime API`][ra] subsystem using [`RuntimeApiRequest::Validators`][ram] and [`RuntimeApiRequest::ValidatorGroups`][ram]
- Determine if the node controls a key in the current validator set. Call this the local key if so.
- If the local key exists, extract the parachain head and validation function from the [`Runtime API`][ra] for the parachain the local key is assigned to by issuing a [`RuntimeApiRequest::Validators`][ram]
- Issue a [`RuntimeApiRequest::SigningContext`][ram] message to get a context that will later be used upon signing.

### On Receiving New Candidate Backing Message

```rust
match msg {
  GetBackedCandidates(hashes, tx) => {
    // Send back a set of backable candidates.
  }
  CandidateBackingMessage::Second(hash, candidate) => {
    if candidate is unknown and in local assignment {
      if spawn_validation_work(candidate, parachain head, validation function).await == Valid {
        send(DistributePoV(pov))
      }
    }
  }
  CandidateBackingMessage::Statement(hash, statement) => {
    // count to the votes on this candidate
    if let Statement::Seconded(candidate) = statement {
      if candidate.parachain_id == our_assignment {
        spawn_validation_work(candidate, parachain head, validation function)
      }
    }
  }
}
```

Add `Seconded` statements and `Valid` statements to a quorum. If quorum reaches validator-group majority, send a [`ProvisionerMessage`][pm]`::ProvisionableData(ProvisionableData::BackedCandidate(CandidateReceipt))` message.
`Invalid` statements that conflict with already witnessed `Seconded` and `Valid` statements for the given candidate, statements that are double-votes, self-contradictions and so on, should result in issuing a [`ProvisionerMessage`][pm]`::MisbehaviorReport` message for each newly detected case of this kind.

### Validating Candidates.

```rust
fn spawn_validation_work(candidate, parachain head, validation function) {
  asynchronously {
    let pov = (fetch pov block).await

    let valid = (validate pov block).await;
    if valid {
      // make PoV available for later distribution. Send data to the availability store to keep.
      // sign and dispatch `valid` statement to network if we have not seconded the given candidate.
    } else {
      // sign and dispatch `invalid` statement to network.
    }
  }
}
```

### Fetch Pov Block

Create a `(sender, receiver)` pair.
Dispatch a [`PoVDistributionMessage`][pdm]`::FetchPoV(relay_parent, candidate_hash, sender)` and listen on the receiver for a response.

### Distribute Pov Block

Dispatch a [`PoVDistributionMessage`][pdm]`::DistributePoV(relay_parent, candidate_descriptor, pov)`.

### Validate PoV Block

Create a `(sender, receiver)` pair.
Dispatch a `CandidateValidationMessage::Validate(validation function, candidate, pov, sender)` and listen on the receiver for a response.

### Distribute Signed Statement

Dispatch a [`StatementDistributionMessage`][pdm]`::Share(relay_parent, SignedFullStatement)`.

[overseersignal]: ../../types/overseer-protocol.md#overseer-signal
[statement]: ../../types/backing.md#statement-type
[stmt]: ../../types/backing.md#statement-type
[csm]: ../../types/overseer-protocol.md#candidate-selection-message
[ram]: ../../types/overseer-protocol.md#runtime-api-message
[cvm]: ../../types/overseer-protocol.md#validation-request-type
[pm]: ../../types/overseer-protocol.md#provisioner-message
[cbm]: ../../types/overseer-protocol.md#candidate-backing-message
[pdm]: ../../types/overseer-protocol.md#pov-distribution-message
[sdm]: ../../types/overseer-protocol.md#statement-distribution-message
[cs]: candidate-selection.md
[cv]: ../utility/candidate-validation.md
[sd]: statement-distribution.md
[ra]: ../utility/runtime-api.md
[pv]: ../utility/provisioner.md
