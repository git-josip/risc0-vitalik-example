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

use std::fs;

use vitalik_example_methods::{VITALIK_EXAMPLE_CALC_ID, VITALIK_EXAMPLE_CALC_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use std::io;

fn main() {
    let mut input = String::new();

    println!("We have math problem =>     x^3 + x + 5 = 35");
    println!("Your solution: ");
    io::stdin().read_line(&mut  input).unwrap();

    let solution_x: u32 = input.trim().parse().unwrap();
    // a new prover is created to run the pw_checker method
    let elf_contents = fs::read(VITALIK_EXAMPLE_CALC_PATH).unwrap();
    let mut prover = Prover::new(&elf_contents, VITALIK_EXAMPLE_CALC_ID).unwrap();

    // Adding input to the prover makes it readable by the guest
    let vec = to_vec(&solution_x).unwrap();
    prover.add_input(&vec).unwrap();

    let receipt = prover.run().unwrap();
    let calculated_solution_for_our_x: u32 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

    println!("Solution for x^3 + x + 5 for our `x` is: {}", &calculated_solution_for_our_x);

    // In most scenarios, we would serialize and send the receipt to a verifier here
    // The verifier checks the receipt with the following call, which panics if the receipt is wrong
    receipt.verify(VITALIK_EXAMPLE_CALC_ID).unwrap();
}
