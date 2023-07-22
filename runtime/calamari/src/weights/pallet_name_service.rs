// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_name_service
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-22, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("/home/runner/runners/2.280.1/_work/Manta/Manta/tests/data/fork.json"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=/home/runner/runners/2.280.1/_work/Manta/Manta/tests/data/fork.json
// --steps=50
// --repeat=40
// --pallet=pallet_name_service
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_name_service.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_name_service.
pub trait WeightInfo {
    fn register() -> Weight;
    fn accept_register() -> Weight;
    fn set_primary_name() -> Weight;
    fn cancel_pending_register() -> Weight;
    fn remove_register() -> Weight;
}

/// Weights for pallet_name_service using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_name_service::WeightInfo for SubstrateWeight<T> {
	// Storage: System Account (r:1 w:1)
	// Storage: NameService PendingRegister (r:1 w:1)
	// Storage: NameService UsernameRecords (r:1 w:0)
	fn register() -> Weight {
		// Minimum execution time: 47_304 nanoseconds.
		Weight::from_ref_time(51_762_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NameService PendingRegister (r:1 w:1)
	// Storage: NameService UsernameRecords (r:0 w:1)
	fn accept_register() -> Weight {
		// Minimum execution time: 26_865 nanoseconds.
		Weight::from_ref_time(27_388_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NameService UsernameRecords (r:1 w:0)
	// Storage: NameService PrimaryRecords (r:1 w:1)
	fn set_primary_name() -> Weight {
		// Minimum execution time: 28_090 nanoseconds.
		Weight::from_ref_time(28_492_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NameService PendingRegister (r:1 w:1)
	fn cancel_pending_register() -> Weight {
		// Minimum execution time: 22_994 nanoseconds.
		Weight::from_ref_time(24_747_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NameService UsernameRecords (r:1 w:1)
	// Storage: NameService PrimaryRecords (r:1 w:0)
	fn remove_register() -> Weight {
		// Minimum execution time: 27_042 nanoseconds.
		Weight::from_ref_time(27_675_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: System Account (r:1 w:1)
	// Storage: NameService PendingRegister (r:1 w:1)
	// Storage: NameService UsernameRecords (r:1 w:0)
	fn register() -> Weight {
		// Minimum execution time: 47_304 nanoseconds.
		Weight::from_ref_time(51_762_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: NameService PendingRegister (r:1 w:1)
	// Storage: NameService UsernameRecords (r:0 w:1)
	fn accept_register() -> Weight {
		// Minimum execution time: 26_865 nanoseconds.
		Weight::from_ref_time(27_388_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: NameService UsernameRecords (r:1 w:0)
	// Storage: NameService PrimaryRecords (r:1 w:1)
	fn set_primary_name() -> Weight {
		// Minimum execution time: 28_090 nanoseconds.
		Weight::from_ref_time(28_492_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: NameService PendingRegister (r:1 w:1)
	fn cancel_pending_register() -> Weight {
		// Minimum execution time: 22_994 nanoseconds.
		Weight::from_ref_time(24_747_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: NameService UsernameRecords (r:1 w:1)
	// Storage: NameService PrimaryRecords (r:1 w:0)
	fn remove_register() -> Weight {
		// Minimum execution time: 27_042 nanoseconds.
		Weight::from_ref_time(27_675_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
