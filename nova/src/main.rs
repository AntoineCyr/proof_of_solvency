use merkle_sum_tree::{Leaf, MerkleSumTree, Position};
use std::env;
use std::{collections::HashMap, env::current_dir, time::Instant};

//use ff::derive::bitvec::vec;
use ff::PrimeField;
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F,
};
use nova_snark::PublicParams;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <function_name>");
        return;
    }

    match args[1].as_str() {
        "inclusion" => inclusion(),
        "liabilities" => liabilities(),
        _ => println!("Unknown function name: {}", args[1]),
    }
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
    let leaf_0 = Leaf::new("0".to_string(), 0);
    let leafs = vec![
        leaf_0.clone(),
        leaf_0.clone(),
        leaf_0.clone(),
        leaf_0.clone(),
    ];
    let leaf_1 = Leaf::new("11672136".to_string(), 10);
    let leaf_2 = Leaf::new("10566265".to_string(), 11);
    let old_merkle_sum_tree = MerkleSumTree::new(leafs).unwrap();
    let mut new_merkle_sum_tree = old_merkle_sum_tree.clone();
    let index = new_merkle_sum_tree.push(leaf_1).unwrap();
    let mut new_merkle_sum_tree2 = new_merkle_sum_tree.clone();
    let index2 = new_merkle_sum_tree2.push(leaf_2).unwrap();
    let merkle_sum_tree_change = MerkleSumTreeChange::new(
        index,
        old_merkle_sum_tree.clone(),
        new_merkle_sum_tree.clone(),
    );
    let merkle_sum_tree_change2 =
        MerkleSumTreeChange::new(index2, new_merkle_sum_tree, new_merkle_sum_tree2);
    let liabilities_input = LiabilitiesInput::new(vec![merkle_sum_tree_change]);
    let liabilities_input2 = LiabilitiesInput::new(vec![merkle_sum_tree_change2]);
    let liabilities = vec![liabilities_input, liabilities_input2];

    for i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert(
            "oldUserHash".to_string(),
            json!(liabilities[i].old_user_hash),
        );
        private_input.insert("oldValues".to_string(), json!([liabilities[i].old_values]));
        private_input.insert(
            "newUserHash".to_string(),
            json!(liabilities[i].new_user_hash),
        );
        private_input.insert("newValues".to_string(), json!(liabilities[i].new_values));
        private_input.insert("tempHash".to_string(), json!(liabilities[i].temp_hash));
        private_input.insert("tempSum".to_string(), json!(liabilities[i].temp_sum));
        private_input.insert(
            "neighborsSum".to_string(),
            json!(liabilities[i].neighbors_sum),
        );
        private_input.insert(
            "neighborsHash".to_string(),
            json!(liabilities[i].neighbor_hash),
        );
        private_input.insert(
            "neighborsBinary".to_string(),
            json!(liabilities[i].neighors_binary),
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
