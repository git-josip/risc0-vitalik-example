// Copyright 2022 Risc0, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let x: u32 = env::read();

    // calculator of: x^3 + x + 5 = 35. hint solution is x = 3
    let solution = x.pow(3) + x + 5;

    if solution != 35 {
        panic!("Invalid solution. Please try again.");
    }

    env::commit(&solution);
}