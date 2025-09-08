pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "../node_modules/circomlib/circuits/comparators.circom";

// Define a template for a Merkle tree circuit
template sumMerkleTree(levels, inputs) {
    // Ensure the number of inputs is a power of 2
    // Define input signals for balances and user hashes
    signal input balance[inputs];
    signal input userHash[inputs];

    // Define output signals for sum and root hash
    signal output sum;
    signal output rootHash;


    // Define arrays for storing sum and hash nodes at each level
    signal sumNodes[levels + 1][inputs];
    signal hashNodes[levels + 1][inputs];

    // Initialize variables
    var levelSize = inputs;
    var nextLevelSize = 0;
    var maxBits = 100;
    var tempNotBig = 0;
    var tempNotNegative = 0;
    
    // Define range check and negative check components
    component rangecheck[inputs];
    component negativecheck[inputs];

    // Loop through each input
    for (var i = 0; i < inputs; i++) {
        // Assign input values to hash and sum nodes
        hashNodes[0][i] <== userHash[i];
        sumNodes[0][i] <== balance[i];

        // Perform range check and negative check
        rangecheck[i] = RangeCheck(maxBits);
        rangecheck[i].in <== balance[i];
        tempNotBig = rangecheck[i].out + tempNotBig;

        negativecheck[i] = NegativeCheck();
        negativecheck[i].in <== balance[i];
        tempNotNegative = negativecheck[i].out + tempNotNegative;
        
        // Assert balance is within range and non-negative
        rangecheck[i].out === 1;
        negativecheck[i].out === 1;
    }

    // Define Merkle sum components
    component merklesum[levels][inputs];
    // Loop through each level
    for (var i = 0; i < levels; i++) {
        // Loop through each pair of nodes at the current level
        for (var j = 0; j < levelSize; j = j + 2) {
            // Compute Merkle sum for each pair of nodes
            merklesum[i][j] = MerkleSum();
            
            // Assign input values to Merkle sum component
            merklesum[i][j].L <== hashNodes[i][j];
            merklesum[i][j].R <== hashNodes[i][j + 1];
            merklesum[i][j].sumL <== sumNodes[i][j];
            merklesum[i][j].sumR <== sumNodes[i][j + 1];

            // Store sum and root hash for the next level
            sumNodes[i + 1][nextLevelSize] <== merklesum[i][j].sum;
            hashNodes[i + 1][nextLevelSize] <== merklesum[i][j].root;
            nextLevelSize += 1;
        }
        // Update the size of the current level
        levelSize = nextLevelSize;
        nextLevelSize = 0;
    }
    // Assign final sum and root hash values
    sum <== sumNodes[levels][0];
    rootHash <== hashNodes[levels][0]; 
}

// Instantiate the main component
component main = sumMerkleTree(2, 4);
