const util = require("util");
const exec = util.promisify(require("child_process").exec);
const snarkjs = require("snarkjs");
import * as fs from "fs";

async function compile(circuit_name: string, signals: any) {
  //await exec(`circom ${circuit_name}.circom --wasm --r1cs -o ./build`);
  console.log("here1");
  await exec(
    "wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_17.ptau"
  );
  console.log("here2");
  await exec(
    `npx snarkjs groth16 setup build/${circuit_name}.r1cs powersOfTau28_hez_final_17.ptau circuit_0000.zkey`
  );
  //npx snarkjs groth16 setup build/liabilities.r1cs powersOfTau28_hez_final_18.ptau circuit_0000.zkey

  let proof_start = Date.now();
  console.log("here3");
  const { proof, publicSignals } = await snarkjs.groth16.fullProve(
    signals,
    `build/${circuit_name}_js/${circuit_name}.wasm`,
    "circuit_0000.zkey"
  );
  console.log(proof);
  console.log(`Proof time: ${Date.now() - proof_start}`);
  await exec(
    "npx snarkjs zkey export verificationkey circuit_0000.zkey verification_key.json"
  );

  const vKey = JSON.parse(fs.readFileSync("verification_key.json", "utf8"));
  let verifier_start = Date.now();
  const res = await snarkjs.groth16.verify(vKey, publicSignals, proof);
  console.log(`Verifier time: ${Date.now() - verifier_start}`);

  if (res === true) {
    console.log("Verification OK");
  } else {
    console.log("Invalid proof");
  }
}

async function main() {
  const size = 32;
  const input = {
    balance: [Array(size).fill(0)],
    userHash: [Array(size).fill(0)],
  };
  compile("liabilities", input);
  //circom liabilities.circom --wasm --r1cs -o ./build
  //npx ts-node circuit_compile.ts
}

main();
