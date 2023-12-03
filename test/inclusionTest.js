const path = require("path");
const assert = require("chai").assert;
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
     it("case I OK", async () => {
        const input = {
            "neighborsSum": ["10","25"],//private
            "neighborsHash": ["11672136","4811434667398150016357199712138626920529027819147804819192874884729019971979"],//private
            "neighborsBinary": ["1","0"],//private
            "sum": "46",//public
            "rootHash": "11346658973375961332326525800941704040239142415932845440524726524725202286597",//public
            "userBalance": "11",//public
            "userEmailHash": "10566265",//public
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"1"});
    }); 

    it("case II wrong rootHash", async () => {
        const input = {
            "neighborsSum": ["10","25"],//private
            "neighborsHash": ["11672136","4811434667398150016357199712138626920529027819147804819192874884729019971979"],//private
            "neighborsBinary": ["1","0"],//private
            "sum": "46",//public
            "rootHash": "434667398150016357199712138626920529027819147804819192874884729019971979",//public
            "userBalance": "11",//public
            "userEmailHash": "10566265",//public
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"0"});
    }); 

    it("case III another user", async () => {
        const input = {
            "neighborsSum": ["10","25"],//private
            "neighborsHash": ["11672136","4811434667398150016357199712138626920529027819147804819192874884729019971979"],//private
            "neighborsBinary": ["1","0"],//private
            "sum": "46",//public
            "rootHash": "434667398150016357199712138626920529027819147804819192874884729019971979",//public
            "userBalance": "11",//public
            "userEmailHash": "214823",//public
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"0"});
    }); 

});
