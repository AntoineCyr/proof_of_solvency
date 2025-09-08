pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "../node_modules/circomlib/circuits/comparators.circom";

// Define a template for a Merkle tree circuit
template sumMerkleTree(levels, inputs) {
    // Constants for balance validation
    var DEFAULT_MAX_BALANCE_BITS = 100;  // Maximum bits for balance (supports up to 2^100)
    // Validate template parameters at compile time
    assert(levels > 0 && levels <= 32);  // Reasonable bounds for tree depth
    assert(inputs > 0 && inputs <= 2**20);  // Reasonable bounds for inputs
    
    // Ensure the number of inputs is a power of 2
    var isPowerOf2 = 1;
    var temp = inputs;
    while (temp > 1) {
        if (temp % 2 == 1) {
            isPowerOf2 = 0;
        }
        temp = temp \ 2;
    }
    assert(isPowerOf2 == 1);  // inputs must be power of 2
    
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
    
    // Define balance validation components
    component balanceCheck[inputs];

    // Loop through each input
    for (var i = 0; i < inputs; i++) {
        // Assign input values to hash and sum nodes
        hashNodes[0][i] <== userHash[i];
        sumNodes[0][i] <== balance[i];

        // Perform non-negative balance validation (allows 0 balances)
        balanceCheck[i] = NonNegativeBalanceCheck(DEFAULT_MAX_BALANCE_BITS);
        balanceCheck[i].balance <== balance[i];
        balanceCheck[i].out === 1;
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
