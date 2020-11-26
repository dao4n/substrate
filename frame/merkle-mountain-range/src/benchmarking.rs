// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Benchmarks for the MMR pallet.

#![cfg_attr(not(feature = "std"), no_std)]

use crate::{*, tests::MMR};
use frame_support::traits::OnInitialize;
use frame_benchmarking::benchmarks;

benchmarks! {
	_ {	}

	on_initialize {
		let x in 0 .. 1_000;

		let block_number = x as u64;
	}: {
		MMR::on_initialize(block_number);
	} verify {
		assert_eq!(crate::NumberOfLeaves::<DefaultInstance>::get(), block_number + 1);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::*;
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_on_initialize::<Test>());
		})
	}
}