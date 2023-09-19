const path = require("path");
const assert = require("chai").assert;
const wasm_tester = require("circom_tester").wasm;

describe("liabilities", () => {
    var circ_file = path.join(__dirname, "circuits", "../../liabilities.circom");
    var circ, num_constraints;

    before(async () => {
        circ = await wasm_tester(circ_file);
        /* await circ.loadConstraints();
        num_constraints = circ.constraints.length;
        console.log("Liabilities #Constraints:", num_constraints, "Expected:", "?"); */
    });

     it("case I OK", async () => {
        const input = {
            "balance": ["10", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"sum": "46", "rootHash": "1925011364609672314997423740918945504928475937983787094612250833114331232382","notNegative":"1","allSmallRange":"1"});
    }); 

    it("case II Negative", async () => {
        const input = {
            "balance": ["-10", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"sum": "26", "rootHash": "9447252380650637689610906684495286616391799236533218562511697775482934690331","notNegative":"0","allSmallRange":"1"});
    }); 

    it("case III BigInt", async () => {
        const input = {
            "balance": ["12676506002282294019603205317092", "11","12","13"],
            "emailHash": ["11672136", "10566265","3423253245","5342523"],
        };
        const witness = await circ.calculateWitness(input, 1);
        await circ.checkConstraints(witness);
        await circ.assertOut(witness, {"sum": "12676506002282294019603205317128", "rootHash": "472361104794911560238043263374509561579994270732052119684567515083505259026","notNegative":"1","allSmallRange":"0"});
    }); 
});
