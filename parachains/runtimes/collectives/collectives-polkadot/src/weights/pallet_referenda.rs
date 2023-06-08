// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_referenda
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_referenda.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: FellowshipCollective Members (r:1 w:0)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumCount (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:0 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `159279`
		// Minimum execution time: 29_859_000 picoseconds.
		Weight::from_parts(30_332_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `317568`
		// Minimum execution time: 54_652_000 picoseconds.
		Weight::from_parts(54_981_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2004`
		//  Estimated: `159279`
		// Minimum execution time: 84_277_000 picoseconds.
		Weight::from_parts(86_396_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2045`
		//  Estimated: `159279`
		// Minimum execution time: 83_603_000 picoseconds.
		Weight::from_parts(85_564_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `317568`
		// Minimum execution time: 124_446_000 picoseconds.
		Weight::from_parts(127_196_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `634`
		//  Estimated: `317568`
		// Minimum execution time: 67_332_000 picoseconds.
		Weight::from_parts(68_060_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `317`
		//  Estimated: `4365`
		// Minimum execution time: 32_800_000 picoseconds.
		Weight::from_parts(33_194_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167`
		//  Estimated: `4365`
		// Minimum execution time: 16_319_000 picoseconds.
		Weight::from_parts(16_595_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `317568`
		// Minimum execution time: 39_829_000 picoseconds.
		Weight::from_parts(40_797_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: FellowshipReferenda MetadataOf (r:1 w:0)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `517`
		//  Estimated: `317568`
		// Minimum execution time: 157_047_000 picoseconds.
		Weight::from_parts(158_391_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:0)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `4277`
		// Minimum execution time: 11_262_000 picoseconds.
		Weight::from_parts(11_561_000, 0)
			.saturating_add(Weight::from_parts(0, 4277))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2385`
		//  Estimated: `159279`
		// Minimum execution time: 70_608_000 picoseconds.
		Weight::from_parts(72_409_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2385`
		//  Estimated: `159279`
		// Minimum execution time: 73_010_000 picoseconds.
		Weight::from_parts(75_118_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1807`
		//  Estimated: `4365`
		// Minimum execution time: 35_429_000 picoseconds.
		Weight::from_parts(36_529_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1774`
		//  Estimated: `4365`
		// Minimum execution time: 35_263_000 picoseconds.
		Weight::from_parts(36_401_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1790`
		//  Estimated: `4365`
		// Minimum execution time: 41_442_000 picoseconds.
		Weight::from_parts(42_495_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1831`
		//  Estimated: `4365`
		// Minimum execution time: 40_284_000 picoseconds.
		Weight::from_parts(41_554_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263`
		//  Estimated: `159279`
		// Minimum execution time: 25_964_000 picoseconds.
		Weight::from_parts(26_388_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `159279`
		// Minimum execution time: 26_428_000 picoseconds.
		Weight::from_parts(26_997_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `4365`
		// Minimum execution time: 18_565_000 picoseconds.
		Weight::from_parts(18_815_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `159279`
		// Minimum execution time: 39_029_000 picoseconds.
		Weight::from_parts(39_327_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `714`
		//  Estimated: `159279`
		// Minimum execution time: 66_856_000 picoseconds.
		Weight::from_parts(69_530_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `767`
		//  Estimated: `159279`
		// Minimum execution time: 87_496_000 picoseconds.
		Weight::from_parts(91_067_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `750`
		//  Estimated: `159279`
		// Minimum execution time: 76_994_000 picoseconds.
		Weight::from_parts(90_054_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `767`
		//  Estimated: `159279`
		// Minimum execution time: 83_812_000 picoseconds.
		Weight::from_parts(85_760_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `771`
		//  Estimated: `159279`
		// Minimum execution time: 56_903_000 picoseconds.
		Weight::from_parts(58_110_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `771`
		//  Estimated: `317568`
		// Minimum execution time: 101_848_000 picoseconds.
		Weight::from_parts(108_838_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `767`
		//  Estimated: `159279`
		// Minimum execution time: 85_974_000 picoseconds.
		Weight::from_parts(87_622_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda MetadataOf (r:0 w:1)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `352`
		//  Estimated: `4365`
		// Minimum execution time: 21_903_000 picoseconds.
		Weight::from_parts(22_357_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda MetadataOf (r:1 w:1)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `4365`
		// Minimum execution time: 19_526_000 picoseconds.
		Weight::from_parts(19_845_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
