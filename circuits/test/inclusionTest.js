const path = require("path");
//const assert = require("chai").assert;
const wasm_tester = require("circom_tester").wasm;

describe("inclusion", () => {
  var circ_file = path.join(__dirname, "circuits", "../../inclusion.circom");
  var circ, num_constraints;

  before(async () => {
    circ = await wasm_tester(circ_file);
    /* await circ.loadConstraints();
        num_constraints = circ.constraints.length;
        console.log("Liabilities #Constraints:", num_constraints, "Expected:", "?"); */
  });
  //use path as private input instead of list of all balances
  //input: previous data, output now data
  it("case I OK", async () => {
    const input = {
      // [{"neighborsBinary": Array [String("1"), String("0")], "userEmailHash": String("10566265"), "neighborsHash": Array [String("11672136"), String("4811434667398150016357199712138626920529027819147804819192874884729019971979")], "userBalance": String("11"), "sum": String("46"), "neighborsSum": Array [String("10"), String("25")], "rootHash": String("11346658973375961332326525800941704040239142415932845440524726524725202286597")}]
      //step_in: ["1", "0", "0", "0", "0"],
      step_in: [
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
      ],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4811434667398150016357199712138626920529027819147804819192874884729019971979",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //private
      rootHash:
        "11346658973375961332326525800941704040239142415932845440524726524725202286597", //private
      userBalance: "11", //private
      userEmailHash: "10566265", //private
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, {
      step_out: [
        "1",
        "46",
        "11346658973375961332326525800941704040239142415932845440524726524725202286597",
        "11",
        "10566265",
        "1",
        "1",
        "11",
        "21",
        "46",
        "10566265",
        "7278075798756016559105993294910877944649956187796712843120467690386329978333",
        "11346658973375961332326525800941704040239142415932845440524726524725202286597",
      ],
    });
  });

  it("case II wrong rootHash", async () => {
    const input = {
      //step_in: ["1", "0", "0", "0", "0"],
      step_in: [
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
      ],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4811434667398150016357199712138626920529027819147804819192874884729019971979",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //public
      rootHash:
        "434667398150016357199712138626920529027819147804819192874884729019971979", //public
      userBalance: "11", //public
      userEmailHash: "10566265", //public
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, { step_out: ["0"] });
  });

  it("case III another user", async () => {
    const input = {
      //step_in: ["1", "0", "0", "0", "0"],
      step_in: [
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
      ],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4811434667398150016357199712138626920529027819147804819192874884729019971979",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //public
      rootHash:
        "434667398150016357199712138626920529027819147804819192874884729019971979", //public
      userBalance: "11", //public
      userEmailHash: "214823", //public
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, { step_out: ["0"] });
  });

  it("case IV Not in previous states but valid", async () => {
    const input = {
      //step_in: ["0", "0", "0", "0", "0"],
      step_in: [
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
      ],
      neighborsSum: ["10", "25"], //private
      neighborsHash: [
        "11672136",
        "4811434667398150016357199712138626920529027819147804819192874884729019971979",
      ], //private
      neighborsBinary: ["1", "0"], //private
      sum: "46", //private
      rootHash:
        "11346658973375961332326525800941704040239142415932845440524726524725202286597", //private
      userBalance: "11", //private
      userEmailHash: "10566265", //private
    };
    const witness = await circ.calculateWitness(input, 1);
    await circ.checkConstraints(witness);
    await circ.assertOut(witness, { step_out: ["1"] });
  });
});
