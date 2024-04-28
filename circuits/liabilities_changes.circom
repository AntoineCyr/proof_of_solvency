//inputs, merkle root, 

pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "../node_modules/circomlib/circuits/comparators.circom";


// Define a template for liabilities proof circuit
template liabilities(levels, changes) {
    // Define inputs
    signal input oldEmailHash[changes];
    signal input oldValues[changes];
    signal input newEmailHash[changes];
    signal input newValues[changes];
    signal input tempHash[changes+1];
    signal input tempSum[changes+1];
    signal input oldSum;
    signal input oldRootHash;
    signal input neighborsSum[changes][levels];
    signal input neighborsHash[changes][levels];
    signal input neighborsBinary[changes][levels];

    // Define outputs
    signal output notNegative;
    signal output allSmallRange;
    signal output validHash;
    signal output validSum;
    signal output newRootHash;
    signal output newSum;

    // Calculate newRootHash and newSum
    newRootHash <== tempHash[changes ];
    newSum <== tempSum[changes];
    oldSum === tempSum[0]
    oldRootHash === tempHash[0]
    
    var currentSum = oldSum;

    // Part 1: Check validity of new values
    var tempNotBig = 0;
    var tempNotNegative = 0;
    var maxBits = 100;

    // Iterate through each change
    for (var i = 0; i < changes; i++) {
        // Calculate currentSum
        currentSum = currentSum + newValues[i] - oldValues[i];

        // Perform range check and negative check
        rangecheck[i] = RangeCheck(maxBits);
        rangecheck[i].in <== newValues[i];
        tempNotBig = rangecheck[i].out + tempNotBig;

        negativecheck[i] = NegativeCheck();
        negativecheck[i].in <== newValues[i];
        tempNotNegative = negativecheck[i].out + tempNotNegative;
    }

    // Check if all new values are within range
    rangeEqual = IsEqual();
    rangeEqual.in <== [changes, tempNotBig];
    allSmallRange <== rangeEqual.out;

    // Check if all new values are not negative
    negativeEqual = IsEqual();
    negativeEqual.in <== [changes, tempNotNegative];
    notNegative <== negativeEqual.out;

    // Check if newSum equals currentSum
    equalSum = IsEqual();
    equalSum.in <== [newSum, currentSum];

    // Part 2: Check validity of old and new paths
    // Ensure that old root + change = temp root
   
    component switcherHash[2][changes][levels];
    component switcherSum[2][changes][levels];
    component merklesum[2][changes][levels];
    component hashEqual[2][changes];
    component sumEqual[2][changes];

    signal tempValidHash[changes+1];
    signal tempValidSum [changes+1];

    tempValidHash[0] <== 1;
    tempValidSum[0] <== 1;

    for (var j = 0; j < changes; j++){
        for  (var i = 0; i<levels; i++){
            switcherHash[0][j][i] = Switcher();
            switcherHash[0][j][i].sel <== neighborsBinary[j][i];
            switcherHash[0][j][i].L <== hashNodes[0][j][i];
            switcherHash[0][j][i].R <== neighborsHash[j][i];

            switcherSum[0][j][i] = Switcher();
            switcherSum[0][j][i].sel <== neighborsBinary[j][i];
            switcherSum[0][j][i].L <== sumNodes[0][j][i];
            switcherSum[0][j][i].R <== neighborsSum[j][i];

            merklesum[0][j][i] = MerkleSum();
            merklesum[0][j][i].L <== switcherHash[0][j][i].outL;
            merklesum[0][j][i].R <== switcherHash[0][j][i].outR;
            merklesum[0][j][i].sumL <== switcherSum[0][j][i].outL;
            merklesum[0][j][i].sumR <== switcherSum[0][j][i].outR;

            sumNodes[0][j][i+1] <== merklesum[0][j][i].sum;
            hashNodes[0][j][i+1] <== merklesum[0][j][i].root;

            switcherHash[1][j][i] = Switcher();
            switcherHash[1][j][i].sel <== neighborsBinary[j][i];
            switcherHash[1][j][i].L <== hashNodes[1][j][i];
            switcherHash[1][j][i].R <== neighborsHash[j][i];

            switcherSum[1][j][i] = Switcher();
            switcherSum[1][j][i].sel <== neighborsBinary[j][i];
            switcherSum[1][j][i].L <== sumNodes[1][j][i];
            switcherSum[1][j][i].R <== neighborsSum[j][i];

            merklesum[1][j][i] = MerkleSum();
            merklesum[1][j][i].L <== switcherHash[1][j][i].outL;
            merklesum[1][j][i].R <== switcherHash[1][j][i].outR;
            merklesum[1][j][i].sumL <== switcherSum[1][j][i].outL;
            merklesum[1][j][i].sumR <== switcherSum[1][j][i].outR;
            
            sumNodes[1][j][i+1] <== merklesum[1][j][i].sum;
            hashNodes[1][j][i+1] <== merklesum[1][j][i].root;
        }

    //value is in old temp hash
    hashEqual[0][j] = IsEqual();
    hashEqual[0][j].in <== [hashNodes[0][j][levels],tempHash[j]];
    tempOldHashEqual[j+1] <== tempOldHashEqual[j] * hashEqual[0][j].out;

    //new temp hash is valid
    hashEqual[1][j] = IsEqual();
    hashEqual[1][j].in <== [hashNodes[1][j][levels],tempHash[j+1]];
    tempValidHash[j+1] <== tempValidHash[j] * hashEqual[1][j].out;

    //old sum is in tempSum
    sumEqual[0][j] = IsEqual();
    sumEqual[0][j].in <== [sumNodes[0][j][levels],tempSum[j]];
    tempOldSumEqual[j+1] <== tempOldSumEqual[j] * sumEqual[0][j].out;

    //new sum is valid
    sumEqual[1][j] = IsEqual();
    sumEqual[1][j].in <== [sumNodes[1][j][levels],tempSum[j+1]];
    tempValidSum[j+1] <==  tempValidSum[j] * sumEqual[1][j].out;
    }


    // Check if tempHash and tempSum are valid
    validHash <== tempValidHash[changes]*tempOldHashEqual[changes];
    validSum <== tempValidSum[changes]*tempOldSumEqual[changes];
}

// Define main component
component main {public [newSum, oldSum]} = liabilities(7, 2);
