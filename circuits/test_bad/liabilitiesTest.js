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
    console.log("Liabilities constraints:", num_constraints);
  });

  //Compare tree as inputs or build tree inside
  it("case I OK", async () => {
    const size = 8192;
    const input = {
      balance: [Array(size).fill(0)],
      emailHash: [Array(size).fill(0)],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      sum: "46",
      rootHash:
        "11346658973375961332326525800941704040239142415932845440524726524725202286597",
      notNegative: "1",
      allSmallRange: "1",
    });
  });
});
