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
      oldEmailHash: ["10566265"],
      oldValues: [11],
      newEmailHash: ["19573022"],
      newValues: [15],
      tempHash: [
        "11346658973375961332326525800941704040239142415932845440524726524725202286597",
        "13409887132926978068627403428641016087864887179975784858831377354067398835782",
      ],
      tempSum: ["46", "50"],
      oldSum: "46",
      oldRootHash:
        "11346658973375961332326525800941704040239142415932845440524726524725202286597",
      neighborsSum: [["10", "25"]],
      neighborsHash: [
        [
          "11672136",
          "4811434667398150016357199712138626920529027819147804819192874884729019971979",
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
