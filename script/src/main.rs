//! A simple script to generate and verify the proof of a given program.

use std::fs::File;
use std::io::Write;

use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    // Generate proof.
    let mut stdin = SP1Stdin::new();

    let n = 10u32;
    stdin.write(&n);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).expect("proving failed");

    // Read output.
    let a = proof.public_values.read::<u128>();
    let b = proof.public_values.read::<u128>();
    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    let buffer: Vec<u8> = serde_json::to_vec(&vk.vk).unwrap();
    println!("stark vk size: {}", buffer.len());
    // let res: Receipt = serde_json::from_slice(&buffer).unwrap();

    let mut file = File::create("stark_vk.bin").unwrap();
    file.write_all(&buffer).unwrap();

    // Save proof.
    proof.save("proof.json").expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
