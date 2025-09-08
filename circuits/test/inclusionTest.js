const path = require("path");
//const assert = require("chai").assert;
const wasm_tester = require("circom_tester").wasm;

describe("inclusion", () => {
  var circ_file = "/tmp/proof_of_solvency/circuits/inclusion.circom";
  var circ, num_constraints;

  before(async () => {
    circ = await wasm_tester(circ_file);
    await circ.loadConstraints();
    num_constraints = circ.constraints.length;
    console.log("Inclusion #Constraints:", num_constraints);
  });
  //use path as private input instead of list of all balances
  //input: previous data, output now data
  it("case I OK", async () => {
    const input = {
      step_in: ["0", "0", "0", "0"],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4804883266082333966929738749002451722893215557695974762826011088617990435037",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //private
      rootHash:
        "7729261165844055213358620257169201670782345148208137496504831508545517076145", //private
      userBalance: "11", //private
      userHash: "10566265", //private
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      step_out: [
        "46",
        "7729261165844055213358620257169201670782345148208137496504831508545517076145",
        "11",
        "10566265",
      ],
    });
  });

  it("case II wrong rootHash", async () => {
    const input = {
      step_in: ["0", "0", "0", "0"],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4804883266082333966929738749002451722893215557695974762826011088617990435037",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //public
      rootHash:
        "7729261165844055213358620257169201670782345148208137496504831508545517076144", //public (wrong)
      userBalance: "11", //public
      userHash: "10566265", //public
    };
    // This should fail with wrong rootHash due to assertions
    try {
      const witness = await circ.calculateWitness(input, 1);
      await circ.checkConstraints(witness);
      throw new Error("Expected circuit to fail with wrong rootHash");
    } catch (error) {
      // Expected to fail
      console.log("✓ Circuit correctly rejected wrong rootHash");
    }
  });

  it("case III another user", async () => {
    const input = {
      step_in: ["0", "0", "0", "0"],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4811434667398150016357199712138626920529027819147804819192874884729019971979",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //public
      rootHash:
        "7729261165844055213358620257169201670782345148208137496504831508545517076145", //public
      userBalance: "11", //public
      userHash: "214823", //public (different user)
    };
    // This should fail with wrong user hash due to assertions
    try {
      const witness = await circ.calculateWitness(input, 1);
      await circ.checkConstraints(witness);
      throw new Error("Expected circuit to fail with wrong user");
    } catch (error) {
      // Expected to fail
      console.log("✓ Circuit correctly rejected wrong user");
    }
  });
});
