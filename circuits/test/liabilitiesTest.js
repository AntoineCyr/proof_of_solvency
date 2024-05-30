const path = require("path");
//const assert = require("chai").assert;
const wasm_tester = require("circom_tester").wasm;

describe("liabilities", () => {
  var circ_file = path.join(__dirname, "circuits", "../../liabilities.circom");
  var circ, num_constraints;

  before(async () => {
    circ = await wasm_tester(circ_file);
    await circ.loadConstraints();
    num_constraints = circ.constraints.length;
    console.log("Liabilities #Constraints:", num_constraints);
  });

  //Compare tree as inputs or build tree inside
  it("case I OK", async () => {
    const input = {
      balance: ["10", "11", "12", "13"],
      userHash: ["11672136", "10566265", "3423253245", "5342523"],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      sum: "46",
      rootHash:
        "7729261165844055213358620257169201670782345148208137496504831508545517076145",
      notNegative: "1",
      allSmallRange: "1",
    });
  });

  it("case II Negative", async () => {
    const input = {
      balance: ["-10", "11", "12", "13"],
      userHash: ["11672136", "10566265", "3423253245", "5342523"],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      sum: "26",
      rootHash:
        "1649484189536111737524036492777494266433459959517777052724263542446495244303",
      notNegative: "0",
      allSmallRange: "1",
    });
  });
});
