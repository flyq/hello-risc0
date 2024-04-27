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

use host::sha2_chain;
use methods::HELLO_GUEST_ID;
use std::fs::File;
use std::io::Write;

fn main() {
    // Pick two numbers
    let input: [u8; 32] = [5; 32];
    let num_iters: u32 = 10;
    let (receipt, _) = sha2_chain(input, num_iters);

    // Here is where one would send 'receipt' over the network...
    println!("receipt: {:?}", receipt.journal.decode::<[u8; 32]>());

    let buffer = serde_json::to_vec(&receipt).unwrap();
    println!("buffer size: {}", buffer.len());
    // let res: Receipt = serde_json::from_slice(&buffer).unwrap();

    let mut file = File::create("receipt.bin").unwrap();
    file.write_all(&buffer).unwrap();
    println!("HELLO_GUEST_ID: {:?}", HELLO_GUEST_ID);

    // Verify receipt, panic if it's wrong
    receipt.verify(HELLO_GUEST_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
