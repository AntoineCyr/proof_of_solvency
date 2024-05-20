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
      emailHash: ["11672136", "10566265", "3423253245", "5342523"],
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

  it("case II Negative", async () => {
    const input = {
      balance: ["-10", "11", "12", "13"],
      emailHash: ["11672136", "10566265", "3423253245", "5342523"],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      sum: "26",
      rootHash:
        "5895925909654415104931655394104674186092723093092091960387529796108826570695",
      notNegative: "0",
      allSmallRange: "1",
    });
  });

  it("case III BigInt", async () => {
    const input = {
      balance: ["12676506002282294019603205317092", "11", "12", "13"],
      emailHash: ["11672136", "10566265", "3423253245", "5342523"],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      sum: "12676506002282294019603205317128",
      rootHash:
        "18385685220892903645862822916010859808752020528765474984030623593947921934339",
      notNegative: "1",
      allSmallRange: "0",
    });
  });

  it("case III all zero", async () => {
    const input = {
      balance: ["0", "0", "0", "0"],
      emailHash: ["0", "0", "0", "0"],
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      sum: "0",
      rootHash:
        "11657350615105339299979493155253703849333416002063536489697607397247709653621",
      notNegative: "1",
      allSmallRange: "1",
    });
  });
});
