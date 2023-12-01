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
    //Compare tree as inputs or build tree inside
    //use path as private input instead of list of all balances
     it("case I OK", async () => {
        const input = {
            "balance": ["10", "11","12","13"],//private
            "emailHash": ["11672136", "10566265","3423253245","5342523"],//private
            "sum": "46",//public
            "rootHash": "11346658973375961332326525800941704040239142415932845440524726524725202286597",//public
            "userBalance": "11",//public
            "userEmailHash": "10566265",//public
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"1","validMerkleTree":"1"});
    }); 

    it("case II not included", async () => {
        const input = {
            "balance": ["10", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
            "sum": "46",
            "rootHash": "11346658973375961332326525800941704040239142415932845440524726524725202286597",
            "userBalance": "11",
            "userEmailHash": "10566265123",
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"0","validMerkleTree":"1"});  }); 

    it("case III wrong balance", async () => {
        const input = {
            "balance": ["10", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
            "sum": "46",
            "rootHash": "11346658973375961332326525800941704040239142415932845440524726524725202286597",
            "userBalance": "13",
            "userEmailHash": "10566265",
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"0","validMerkleTree":"1"});}); 

    it("case IV sum not valid", async () => {
        const input = {
            "balance": ["10", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
            "sum": "200",
            "rootHash": "11346658973375961332326525800941704040239142415932845440524726524725202286597",
            "userBalance": "11",
            "userEmailHash": "10566265",
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"1","validMerkleTree":"0"});}); 

    it("case V merkle root not valid", async () => {
        const input = {
            "balance": ["10", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
            "sum": "46",
            "rootHash": "1925011364609672314997423740918945504937983787094612250833114331232382",
            "userBalance": "11",
            "userEmailHash": "10566265",
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"inMerkleTree":"1","validMerkleTree":"0"});}); 

});
