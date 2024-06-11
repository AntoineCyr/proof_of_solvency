use merkle_sum_tree::{Leaf, MerkleSumTree, Position};
use std::{collections::HashMap, env::current_dir, time::Instant};

//use ff::derive::bitvec::vec;
use ff::PrimeField;
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

#[derive(Debug, Clone)]
pub struct MerkleSumTreeChange {
    index: usize,
    old_merkle_tree: MerkleSumTree,
    new_merkle_tree: MerkleSumTree,
}

impl MerkleSumTreeChange {
    pub fn new(
        index: usize,
        old_merkle_tree: MerkleSumTree,
        new_merkle_tree: MerkleSumTree,
    ) -> MerkleSumTreeChange {
        MerkleSumTreeChange {
            index,
            old_merkle_tree,
            new_merkle_tree,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiabilitiesInput {
    old_user_hash: Vec<String>,
    old_values: Vec<i32>,
    new_user_hash: Vec<String>,
    new_values: Vec<i32>,
    temp_hash: Vec<String>,
    temp_sum: Vec<i32>,
    neighbors_sum: Vec<Vec<i32>>,
    neighbor_hash: Vec<Vec<String>>,
    neighors_binary: Vec<Vec<i32>>,
}

impl LiabilitiesInput {
    pub fn new(changes: Vec<MerkleSumTreeChange>) -> LiabilitiesInput {
        let mut old_user_hash = vec![];
        let mut old_values = vec![];
        let mut new_user_hash = vec![];
        let mut new_values = vec![];
        let mut temp_hash = vec![];
        let mut temp_sum = vec![];
        let mut neighbors_sum = vec![];
        let mut neighbor_hash = vec![];
        let mut neighors_binary = vec![];

        temp_hash.push(
            changes[0]
                .old_merkle_tree
                .get_root_hash()
                .unwrap()
                .to_string(),
        );
        temp_sum.push(changes[0].old_merkle_tree.get_root_sum().unwrap());
        for change in changes {
            let old_leaf = change.old_merkle_tree.get_leaf(change.index).unwrap();
            let new_leaf = change.new_merkle_tree.get_leaf(change.index).unwrap();
            let old_merkle_path = change
                .old_merkle_tree
                .get_proof(change.index)
                .unwrap()
                .unwrap()
                .get_path();
            let new_merkle_path = change
                .old_merkle_tree
                .get_proof(change.index)
                .unwrap()
                .unwrap()
                .get_path();
            assert!(old_merkle_path == new_merkle_path);
            old_user_hash.push(old_leaf.get_node().get_hash().to_string());
            old_values.push(old_leaf.get_node().get_value());
            new_user_hash.push(new_leaf.get_node().get_hash().to_string());
            new_values.push(new_leaf.get_node().get_value());
            temp_hash.push(change.new_merkle_tree.get_root_hash().unwrap().to_string());
            temp_sum.push(change.new_merkle_tree.get_root_sum().unwrap());
            let mut neighbors_sum_change = vec![];
            let mut neighbor_hash_change = vec![];
            let mut neighors_binary_change = vec![];
            for neighbor in old_merkle_path {
                neighbors_sum_change.push(neighbor.get_node().get_value());
                neighbor_hash_change.push(neighbor.get_node().get_hash().to_string());
                match neighbor.get_position() {
                    Position::Left => neighors_binary_change.push(1),
                    Position::Right => neighors_binary_change.push(0),
                }
            }
            neighbors_sum.push(neighbors_sum_change);
            neighbor_hash.push(neighbor_hash_change);
            neighors_binary.push(neighors_binary_change);
        }

        let liabilities_proof = LiabilitiesInput {
            old_user_hash,
            old_values,
            new_user_hash,
            new_values,
            temp_hash,
            temp_sum,
            neighbors_sum,
            neighbor_hash,
            neighors_binary,
        };
        liabilities_proof
    }
}

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
    let circuit_filepath = root.join("../circuits/compile/inclusion.r1cs");
    let witness_generator_file = root.join("../circuits/compile/inclusion_js/inclusion.wasm");
    let root = current_dir().unwrap();

    let circuit_file = root.join(circuit_filepath);
    println!("{:?}", circuit_file);
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));

    //let pp = create_public_params::<G1, G2>(r1cs.clone());
    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());

    let mut private_inputs = Vec::new();

    for _i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert("neighborsSum".to_string(), json!([10, 25]));
        private_input.insert(
            "neighborsHash".to_string(),
            json!([
                "3677691099277992195".to_string(),
                "28561926254282265537438209390008430313500190636100879443084340322545760566831"
                    .to_string()
            ]),
        );
        private_input.insert("neighborsBinary".to_string(), json!([1, 0]));
        private_input.insert("sum".to_string(), json!(46));
        private_input.insert(
            "rootHash".to_string(),
            json!(
                "7270102280961693760725023799639149982274443118879847539912228780362948820462"
                    .to_string()
            ),
        );
        private_input.insert("userBalance".to_string(), json!(11));
        private_input.insert(
            "userHash".to_string(),
            json!("13892846547337029487".to_string()),
        );
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
    let z0_secondary = [F::<G2>::from(0)];
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, iteration_count, &start_public_input, &z0_secondary);
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    println!("res {:?}", res.is_ok())
}

fn main() {
    //inclusion();
    liabilities();
}

fn liabilities() {
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();
    let circuit_file = root.join("../circuits/compile/liabilities_changes_folding.r1cs");
    let witness_generator_file = root.join(
        "../circuits/compile/liabilities_changes_folding_js/liabilities_changes_folding.wasm",
    );

    println!("{:?}", circuit_file);
    let start_proof = Instant::now();
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));

    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());
    let iteration_count = 1;
    let mut private_inputs = Vec::new();

    for i in 0..iteration_count {
        let leaf_0 = Leaf::new("0".to_string(), 0);
        let mut leafs = vec![
            leaf_0.clone(),
            leaf_0.clone(),
            leaf_0.clone(),
            leaf_0.clone(),
        ];
        let leaf_1 = Leaf::new("11672136".to_string(), 10);
        let old_merkle_sum_tree = MerkleSumTree::new(leafs).unwrap();
        let mut new_merkle_sum_tree = old_merkle_sum_tree.clone();
        new_merkle_sum_tree.push(leaf_1);
        let merkle_sum_tree_change =
            MerkleSumTreeChange::new(0, old_merkle_sum_tree, new_merkle_sum_tree);
        let liabilities_input = LiabilitiesInput::new(vec![merkle_sum_tree_change]);

        let mut private_input = HashMap::new();
        private_input.insert(
            "oldUserHash".to_string(),
            json!(liabilities_input.old_user_hash),
        );
        private_input.insert(
            "oldValues".to_string(),
            json!([liabilities_input.old_values]),
        );
        private_input.insert(
            "newUserHash".to_string(),
            json!(liabilities_input.new_user_hash),
        );
        private_input.insert("newValues".to_string(), json!(liabilities_input.new_values));
        println!("{:?}", liabilities_input.temp_hash);
        private_input.insert("tempHash".to_string(), json!(liabilities_input.temp_hash));
        private_input.insert("tempSum".to_string(), json!(liabilities_input.temp_sum));
        private_input.insert(
            "neighborsSum".to_string(),
            json!(liabilities_input.neighbors_sum),
        );
        private_input.insert(
            "neighborsHash".to_string(),
            json!(liabilities_input.neighbor_hash),
        );
        private_input.insert(
            "neighborsBinary".to_string(),
            json!(liabilities_input.neighors_binary),
        );
        private_inputs.push(private_input);
    }

    let start_public_input = [
        F::<G1>::from(1),
        F::<G1>::from(1),
        F::<G1>::from_str_vartime(
            "9577381579138472660640687876808526267294443674721706486194556629472463835227",
        )
        .unwrap(),
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
