pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "../node_modules/circomlib/circuits/comparators.circom";


template sumMerkleTree(levels,inputs) {
    //number of inputs need to be == 2^n
    
    signal input balance[inputs];
    signal input emailHash[inputs];

    signal output sum;
    signal output rootHash;

    signal output notNegative;
    signal output allSmallRange;

    signal sumNodes[levels+1][inputs];
    signal hashNodes[levels+1][inputs];

    var levelSize = inputs;
    var nextLevelSize = 0;
    var maxBits = 100;
    var tempNotBig = 0;
    var tempNotNegative = 0;
    
    component rangecheck[inputs];
    component negativecheck[inputs];

    for (var i = 0; i<inputs; i++){
        hashNodes[0][i] <== emailHash[i];
        sumNodes[0][i] <== balance[i];

        rangecheck[i] = RangeCheck(maxBits);
        rangecheck[i].in <== balance[i];
        tempNotBig = rangecheck[i].out + tempNotBig;

        negativecheck[i] = NegativeCheck();
        negativecheck[i].in <== balance[i];
        tempNotNegative = negativecheck[i].out + tempNotNegative;
    }
    component rangeEqual = IsEqual();
    rangeEqual.in <== [inputs,tempNotBig];
    allSmallRange <== rangeEqual.out;

    component negativeEqual = IsEqual();
    negativeEqual.in <== [inputs,tempNotNegative];
    notNegative <== negativeEqual.out;

    component merklesum[levels][inputs];
    for (var i = 0; i < levels; i++) {
        for (var j = 0; j< levelSize; j = j+2)  {
            merklesum[i][j] = MerkleSum();
            
            merklesum[i][j].L <== hashNodes[i][j];
            merklesum[i][j].R <== hashNodes[i][j+1];
            merklesum[i][j].sumL <== sumNodes[i][j];
            merklesum[i][j].sumR <== sumNodes[i][j+1];

            
            sumNodes[i+1][nextLevelSize] <== merklesum[i][j].sum;
            hashNodes[i+1][nextLevelSize] <== merklesum[i][j].root;
            nextLevelSize +=1;
        }
        levelSize = nextLevelSize;
        nextLevelSize = 0;
    }
    sum <== sumNodes[levels][0];
    rootHash <== hashNodes[levels][0]; 
}

component main = sumMerkleTree(2,4);