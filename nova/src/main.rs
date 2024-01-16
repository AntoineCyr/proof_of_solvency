use std::{collections::HashMap, env::current_dir, time::Instant};


use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F, S,
};

use nova_snark::{
    provider,
    pasta_curves,
    traits::{circuit::StepCircuit, Group},
    CompressedSNARK, PublicParams,
};


fn main() {
    println!("Hello, world!");

    // The cycle of curves we use, can be any cycle supported by Nova
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();
    let circuit_file = root.join("examples/bitcoin/circom/bitcoin_benchmark.r1cs");
    let witness_generator_file =
        root.join("examples/bitcoin/circom/bitcoin_benchmark_cpp/bitcoin_benchmark");

    let r1cs = load_r1cs::<G1, G2>(&circuit_file); // loads R1CS file into memory
}
