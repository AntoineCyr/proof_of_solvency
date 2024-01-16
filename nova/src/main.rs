fn main() {
    // The cycle of curves we use, can be any cycle supported by Nova
type G1 = pasta_curves::pallas::Point;
type G2 = pasta_curves::vesta::Point;

let circuit_file = root.join("examples/bitcoin/circom/bitcoin_benchmark.r1cs");
let witness_generator_file =
    root.join("examples/bitcoin/circom/bitcoin_benchmark_cpp/bitcoin_benchmark");

let r1cs = load_r1cs::<G1, G2>(&circuit_file); // loads R1CS file into memory
}
