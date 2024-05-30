const path = require("path");
//const assert = require("chai").assert;
const wasm_tester = require("circom_tester").wasm;

describe("liabilitiesChangeFolding", () => {
  var circ_file = path.join(
    __dirname,
    "circuits",
    "../../liabilities_changes_folding.circom"
  );
  var circ, num_constraints;

  before(async () => {
    circ = await wasm_tester(circ_file);
    await circ.loadConstraints();
    num_constraints = circ.constraints.length;
    console.log("Liabilities #Constraints:", num_constraints);
  });
  it("case I OK", async () => {
    const input = {
      step_in: [
        "1",
        "1",
        "12976641360266753079598818899014603117780494854743518175634070759643875859269",
        "0",
      ],
      oldUserHash: ["18187302216140149989"],
      oldValues: [0],
      newUserHash: ["2060978228548495531"],
      newValues: [10],
      tempHash: [
        "12976641360266753079598818899014603117780494854743518175634070759643875859269",
        "17664979870515770742457044900297477064510829622182688926523338458766575076425",
      ],
      tempSum: ["0", "10"],
      neighborsSum: [["0", "0", "0"]],
      neighborsHash: [
        [
          "18187302216140149989",
          "1238917015649242474601069541970638055803108247773110607513160413661909691384",
          "1983745967305507879327186969997252799225875461519857271723902240176167222148",
        ],
      ],
      neighborsBinary: [["0", "0", "0"]],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);

    await circ.assertOut(witness, {
      step_out: [
        "1",
        "1",
        "17664979870515770742457044900297477064510829622182688926523338458766575076425",
        "10",
      ],
    });
  });
});
