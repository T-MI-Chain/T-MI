// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of tmi.

// tmi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// tmi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tmi.  If not, see <http://www.gnu.org/licenses/>.

//! Runtime modules for parachains code.
//!
//! It is crucial to include all the modules from this crate in the runtime, in
//! particular the `Initializer` module, as it is responsible for initializing the state
//! of the other modules.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod configuration;
pub mod inclusion;
pub mod inclusion_inherent;
pub mod initializer;
pub mod paras;
pub mod scheduler;
pub mod session_info;
pub mod origin;
pub mod dmp;
pub mod ump;
pub mod hrmp;
pub mod reward_points;

pub mod runtime_api_impl;

mod util;

#[cfg(test)]
mod mock;

pub use origin::{Origin, ensure_parachain};

/// Schedule a para to be initialized at the start of the next session with the given genesis data.
pub fn schedule_para_initialize<T: paras::Config>(
	id: primitives::v1::Id,
	genesis: paras::ParaGenesisArgs,
) {
	<paras::Module<T>>::schedule_para_initialize(id, genesis);
}

/// Schedule a para to be cleaned up at the start of the next session.
pub fn schedule_para_cleanup<T>(id: primitives::v1::Id)
where
	T: paras::Config
	+ dmp::Config
	+ ump::Config
	+ hrmp::Config,
{
	<paras::Module<T>>::schedule_para_cleanup(id);
	<dmp::Module<T>>::schedule_para_cleanup(id);
	<ump::Module<T>>::schedule_para_cleanup(id);
	<hrmp::Module<T>>::schedule_para_cleanup(id);
}
