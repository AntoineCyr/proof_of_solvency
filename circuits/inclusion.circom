//inputs, merkle root, 

pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "./node_modules/circomlib/circuits/comparators.circom";


template inclusion(levels) {
    //number of inputs need to be == 2^n
    
    signal input neighborsSum[levels];
    signal input neighborsHash[levels];
    signal input neighborsBinary[levels];
    signal input sum;
    signal input rootHash;
    signal input userBalance;
    signal input userEmailHash;
    signal output inMerkleTree;

    signal sumNodes[levels+1];
    signal hashNodes[levels+1];
    signal rootHashCalc;
    signal sumCalc;
    signal validHash;
    signal validSum;
    sumNodes[0] <== userBalance;
    hashNodes[0] <== userEmailHash;

    component switcherHash[levels];
    component switcherSum[levels];
    component merklesum[levels];
    for  (var i = 0; i<levels; i++){
        switcherHash[i] = Switcher();
        switcherHash[i].sel <== neighborsBinary[i];
        switcherHash[i].L <== hashNodes[i];
        switcherHash[i].R <== neighborsHash[i];

        switcherSum[i] = Switcher();
        switcherSum[i].sel <== neighborsBinary[i];
        switcherSum[i].L <== sumNodes[i];
        switcherSum[i].R <== neighborsSum[i];


        merklesum[i] = MerkleSum();
        merklesum[i].L <== switcherHash[i].outL;
        merklesum[i].R <== switcherHash[i].outR;
        merklesum[i].sumL <== switcherSum[i].outL;
        merklesum[i].sumR <== switcherSum[i].outR;

        
        sumNodes[i+1] <== merklesum[i].sum;
        hashNodes[i+1] <== merklesum[i].root;
    }

    component hashEqual = IsEqual();
    hashEqual.in <== [hashNodes[levels],rootHash];
    validHash <== hashEqual.out;

    component sumEqual = IsEqual();
    sumEqual.in <== [sumNodes[levels],sum];
    validSum <== sumEqual.out;
    
    inMerkleTree <== validSum * validHash;

}

component main = inclusion(2);