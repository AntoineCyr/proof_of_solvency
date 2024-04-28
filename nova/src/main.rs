use std::{collections::HashMap, env::current_dir, time::Instant};

use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F, S,
};
use nova_snark::{
    //provider,
    //traits::{circuit::StepCircuit, Group},
    CompressedSNARK,
    PublicParams,
};
use serde_json::json;

fn run_test(circuit_filepath: String, witness_gen_filepath: String) {
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    println!(
        "Running test with witness generator: {} and group: {}",
        witness_gen_filepath,
        std::any::type_name::<G1>()
    );
    let iteration_count = 5;
    let root = current_dir().unwrap();

    let circuit_file = root.join(circuit_filepath);
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file = root.join(witness_gen_filepath);

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert("adder".to_string(), json!(i));
        private_inputs.push(private_input);
    }

    let start_public_input = [F::<G1>::from(10), F::<G1>::from(10)];

    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());

    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    println!("Creating a RecursiveSNARK...");

    let new_file = FileLocation::PathBuf(witness_generator_file.clone());

    println!("after check");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file),
        r1cs,
        private_inputs,
        start_public_input.to_vec(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    // TODO: empty?
    let z0_secondary = [F::<G2>::from(0)];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, iteration_count, &start_public_input, &z0_secondary);
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());

    // produce a compressed SNARK
    println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let start = Instant::now();

    let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();
    let res = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark);
    println!(
        "CompressedSNARK::prove: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
    let compressed_snark = res.unwrap();

    // verify the compressed SNARK
    println!("Verifying a CompressedSNARK...");
    let start = Instant::now();
    let res = compressed_snark.verify(
        &vk,
        iteration_count,
        start_public_input.to_vec(),
        z0_secondary.to_vec(),
    );
    println!(
        "CompressedSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
}

fn inclusion() {
    let iteration_count = 1;
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();
    let circuit_filepath = root.join("../compile/inclusion.r1cs");
    let witness_generator_file = root.join("../compile/inclusion_js/inclusion.wasm");
    let root = current_dir().unwrap();

    let circuit_file = root.join(circuit_filepath);
    println!("{:?}", circuit_file);
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));

    //let pp = create_public_params::<G1, G2>(r1cs.clone());
    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert("neighborsSum".to_string(), json!([10, 25]));
        private_input.insert(
            "neighborsHash".to_string(),
            json!([
                11672136,
                "4811434667398150016357199712138626920529027819147804819192874884729019971979"
                    .to_string(),
            ]),
        );
        private_input.insert("neighborsBinary".to_string(), json!([1, 0]));
        private_input.insert("sum".to_string(), json!(46));
        private_input.insert(
            "rootHash".to_string(),
            json!(
                "11346658973375961332326525800941704040239142415932845440524726524725202286597"
                    .to_string()
            ),
        );
        private_input.insert("userBalance".to_string(), json!(11));
        private_input.insert("userEmailHash".to_string(), json!(10566265));
        private_inputs.push(private_input);
    }

    let start_public_input = [
        F::<G1>::from(0),
        F::<G1>::from(0),
        F::<G1>::from(0),
        F::<G1>::from(0),
        F::<G1>::from(0),
    ];

    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file),
        r1cs,
        private_inputs,
        start_public_input.to_vec(),
        &pp,
    )
    .unwrap();
    // TODO: empty?
    let z0_secondary = [F::<G2>::from(0)];
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, iteration_count, &start_public_input, &z0_secondary);

    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    let verifier_time = start.elapsed();
    assert!(res.is_ok());
}
fn main() {
    //liabilities changes folding
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();
    let circuit_file = root.join("../circuits/liabilities_changes_folding.r1cs");
    let witness_generator_file =
        root.join("../circuits/liabilities_changes_folding_js/liabilities_changes_folding.wasm");

    println!("{:?}", circuit_file);
    let start_proof = Instant::now();
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));

    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());
    let iteration_count = 20;
    let changes = 32;
    let levels = 20;
    let changes_vec = vec![0; changes];
    let levels_vec = vec![&changes_vec; levels];

    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert("newRootHash".to_string(), json!(11));
        private_input.insert("newSum".to_string(), json!(10566265));
        private_input.insert("oldEmailHash".to_string(), json!(&changes_vec));
        private_input.insert("oldValues".to_string(), json!(&changes_vec));
        private_input.insert("newEmailHash".to_string(), json!(&changes_vec));
        private_input.insert("newValues".to_string(), json!(&changes_vec));
        private_input.insert("tempHash".to_string(), json!(&changes_vec));
        private_input.insert("tempSum".to_string(), json!(&changes_vec));
        private_input.insert("neighborsSum".to_string(), json!(&levels_vec));
        private_input.insert("neighborsHash".to_string(), json!(&levels_vec));
        private_input.insert("neighborsBinary".to_string(), json!(&levels_vec));

        private_inputs.push(private_input);
    }

    let start_public_input = [
        F::<G1>::from(0),
        F::<G1>::from(0),
        F::<G1>::from(0),
        F::<G1>::from(0),
    ];
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file),
        r1cs,
        private_inputs,
        start_public_input.to_vec(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK::proof took {:?}", start_proof.elapsed());
    let z0_secondary = [F::<G2>::from(0)];
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, iteration_count, &start_public_input, &z0_secondary);

    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());
}
