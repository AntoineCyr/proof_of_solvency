const path = require("path");
//const assert = require("chai").assert;
const wasm_tester = require("circom_tester").wasm;

describe("liabilitiesChange", () => {
  var circ_file = path.join(
    __dirname,
    "circuits",
    "../../liabilities_changes.circom"
  );
  var circ, num_constraints;

  before(async () => {
    circ = await wasm_tester(circ_file);
    await circ.loadConstraints();
    num_constraints = circ.constraints.length;
    console.log("Liabilities change constraints:", num_constraints);
  });
  it("case I OK", async () => {
    const change = 82;
    const level = 13;
    const input = {
      oldEmailHash: Array(change).fill(0),
      oldValues: Array(change).fill(0),
      newEmailHash: Array(change).fill(0),
      newValues: Array(change).fill(0),
      tempHash: Array(change).fill(0),
      tempSum: Array(change).fill(0),
      newRootHash:
        "13409887132926978068627403428641016087864887179975784858831377354067398835782",
      newSum: "50",
      oldSum: "46",
      oldRootHash:
        "11346658973375961332326525800941704040239142415932845440524726524725202286597",
      neighborsSum: [Array(change).fill(Array(level).fill(0))],
      neighborsHash: [Array(change).fill(Array(level).fill(0))],
      neighborsBinary: [Array(change).fill(Array(level).fill(0))],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);

    await circ.assertOut(witness, {
      notNegative: "1",
      allSmallRange: "1",
      validHash: "1",
      validSum: "1",
    });
  });
});
