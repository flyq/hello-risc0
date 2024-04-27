// Copyright 2024 RISC Zero, Inc.
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

#![doc = include_str!("../../README.md")]

use methods::HELLO_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// This is a Hello World demo for the RISC Zero zkVM.
// By running the demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a*b == 391.
// The factors a and b are kept secret.

// Compute the product a*b inside the zkVM
pub fn sha2_chain(a: [u8; 32], b: u32) -> (Receipt, [u8; 32]) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: [u8; 32] = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    println!("hash result: {:?}, and I can prove it!", c);

    (receipt, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let bytes: [u8; 32] = [5; 32];
        let num: u32 = 10;
        let (_, result) = sha2_chain(bytes, num);
        assert_eq!(
            result,
            [
                248, 73, 214, 115, 37, 250, 207, 4, 23, 123, 198, 99, 178, 220, 84, 64, 81, 131,
                28, 88, 158, 245, 129, 212, 18, 242, 235, 164, 72, 52, 231, 124
            ],
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}
