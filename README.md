# tmi

Implementation of a https://tmi.network node in Rust based on the Substrate framework.

> **NOTE:** In 2018, we split our implementation of "tmi" from its development framework
> "Substrate". See the [Substrate][substrate-repo] repo for git history prior to 2018.

[substrate-repo]: https://github.com/tmi/substrate

This repo contains runtimes for the tmi, Kusama, and Westend networks. The README provides
information about installing the `tmi` binary and developing on the codebase. For more
specific guides, like how to be a validator, see the
[tmi Wiki](https://wiki.tmi.network/docs/en/).

## Installation

If you just wish to run a tmi node without compiling it yourself, you may
either run the latest binary from our
[releases](https://github.com/tmi/tmi/releases) page, or install
tmi from one of our package repositories.

Installation from the debian or rpm repositories will create a `systemd`
service that can be used to run a tmi node. This is disabled by default,
and can be started by running `systemctl start tmi` on demand (use
`systemctl enable tmi` to make it auto-start after reboot). By default, it
will run as the `tmi` user. Command-line flags passed to the binary can
be customised by editing `/etc/default/tmi`. This file will not be
overwritten on updating tmi. You may also just run the node directly from
the command-line.

### Debian-based (Debian, Ubuntu)

Currently supports Debian 10 (Buster) and Ubuntu 20.04 (Focal), and
derivatives. Run the following commands as the `root` user.

```
# Import the security@parity.io GPG key
gpg --recv-keys --keyserver hkps://keys.mailvelope.com 9D4B2B6EB8F97156D19669A9FF0812D491B96798
gpg --export 9D4B2B6EB8F97156D19669A9FF0812D491B96798 > /usr/share/keyrings/parity.gpg
# Add the Parity repository and update the package index
echo 'deb [signed-by=/usr/share/keyrings/parity.gpg] https://releases.parity.io/deb release main' > /etc/apt/sources.list.d/parity.list
apt update
# Install the `parity-keyring` package - This will ensure the GPG key
# used by APT remains up-to-date
apt install parity-keyring
# Install tmi
apt install tmi

```

### RPM-based (Fedora, CentOS)

Currently supports Fedora 32 and CentOS 8, and derivatives.

```
# Install dnf-plugins-core (This might already be installed)
dnf install dnf-plugins-core
# Add the repository and enable it
dnf config-manager --add-repo https://releases.parity.io/rpm/tmi.repo
dnf config-manager --set-enabled tmi
# Install tmi (You may have to confirm the import of the GPG key, which
# should have the following fingerprint: 9D4B2B6EB8F97156D19669A9FF0812D491B96798)
dnf install tmi
```

## Building

### Install via Cargo

If you want to install tmi in your PATH, you can do so with with:

```bash
cargo install --git https://github.com/tmi/tmi --tag <version> tmi --locked
```

### Build from Source

If you'd like to build from source, first install Rust. You may need to add Cargo's bin directory
to your PATH environment variable. Restarting your computer will do this for you automatically.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software:

```bash
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
git checkout <latest tagged release>
./scripts/init.sh
cargo build --release
```

Note that compilation is a memory intensive process. We recommend having 4 GiB of phyiscal RAM or swap available (keep in mind that if a build hits swap it tends to be very slow).

## Networks

This repo supports runtimes for tmi, Kusama, and Westend.

### Connect to tmi Mainnet

Connect to the global tmi Mainnet network by running:

```bash
./target/release/tmi --chain=tmi
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.tmi.io/#list/tmi

### Connect to the "Kusama" Canary Network

Connect to the global Kusama canary network by running:

```bash
./target/release/tmi --chain=kusama
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.tmi.io/#list/Kusama

### Connect to the Westend Testnet

Connect to the global Westend testnet by running:

```bash
./target/release/tmi --chain=westend
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.tmi.io/#list/Westend

### Obtaining DOTs

If you want to do anything on tmi, Kusama, or Westend, then you'll need to get an account and
some DOT, KSM, or WND tokens, respectively. See the
[claims instructions](https://claims.tmi.network/) for tmi if you have DOTs to claim. For
Westend's WND tokens, see the faucet
[instructions](https://wiki.tmi.network/docs/en/learn-DOT#getting-westies) on the Wiki.

## Hacking on tmi

If you'd actually like to hack on tmi, you can grab the source code and build it. Ensure you have
Rust and the support software installed. This script will install or update Rust and install the
required dependencies (this may take up to 30 minutes on Mac machines):

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

Then, grab the tmi source code:

```bash
git clone https://github.com/tmi/tmi.git
cd tmi
```

Then build the code. You will need to build in release mode (`--release`) to start a network. Only
use debug mode for development (faster compile times for development and testing).

```bash
./scripts/init.sh   # Install WebAssembly. Update Rust
cargo build # Builds all native code
```

You can run the tests if you like:

```bash
cargo test --all
```

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev
```

### Development

You can run a simple single-node development "network" on your machine by running:

```bash
tmi --dev
```

You can muck around by heading to https://tmi.js.org/apps and choose "Local Node" from the
Settings menu.

### Local Two-node Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a
local testnet. You'll need two terminals open. In one, run:

```bash
tmi --chain=tmi-local --alice -d /tmp/alice
```

And in the other, run:

```bash
tmi --chain=tmi-local --bob -d /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/ALICE_BOOTNODE_ID_HERE'
```

Ensure you replace `ALICE_BOOTNODE_ID_HERE` with the node ID from the output of the first terminal.

### Using Docker

[Using Docker](doc/docker.md)

### Shell Completion

[Shell Completion](doc/shell-completion.md)

## Contributing

### Contributing Guidelines

[Contribution Guidelines](CONTRIBUTING.md)

### Contributor Code of Conduct

[Code of Conduct](CODE_OF_CONDUCT.md)

## License

tmi is [GPL 3.0 licensed](LICENSE).

## Important Notice

https://tmi.network/testnetdisclaimer
