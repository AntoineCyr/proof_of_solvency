//inputs, merkle root, 

pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "../node_modules/circomlib/circuits/comparators.circom";


template inclusion(levels) {
    //number of inputs need to be == 2^n
    
    signal input neighborsSum[levels];
    signal input neighborsHash[levels];
    signal input neighborsBinary[levels];
    signal input step_in[5];
    signal input sum;
    signal input rootHash;
    signal input userBalance;
    signal input userEmailHash;
    signal output step_out[5];
    step_out[1] <== sum;
    step_out[2] <== rootHash;
    step_out[3] <== userBalance;
    step_out[4] <== userEmailHash;

    signal sumNodes[levels+1];
    signal hashNodes[levels+1];
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
    
    //signal inMerkleTreeAndValidSum <== validSum * inMerkleTree;
    //step_out[0] <== inMerkleTree * validHash;
    step_out[0] <== validSum * validHash;
}

component main {public [step_in]}= inclusion(2);