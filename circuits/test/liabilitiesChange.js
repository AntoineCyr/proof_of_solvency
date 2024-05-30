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
    console.log("Liabilities Change #Constraints:", num_constraints);
  });
  it("case I OK", async () => {
    const input = {
      oldUserHash: ["10566265"],
      oldValues: [11],
      newUserHash: ["19573022"],
      newValues: [15],
      tempHash: [
        "7729261165844055213358620257169201670782345148208137496504831508545517076145",
        "18385639392567322859359258022392238054588079328206478535947843108833814699484",
      ],
      tempSum: ["46", "50"],
      oldSum: "46",
      oldRootHash:
        "7729261165844055213358620257169201670782345148208137496504831508545517076145",
      neighborsSum: [["10", "25"]],
      neighborsHash: [
        [
          "11672136",
          "4804883266082333966929738749002451722893215557695974762826011088617990435037",
        ],
      ],
      neighborsBinary: [["1", "0"]],
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
