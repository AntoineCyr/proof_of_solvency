//inputs, merkle root, 

pragma circom 2.0.0;
include "./merkle.circom";
include "./utils.circom";
include "./node_modules/circomlib/circuits/comparators.circom";


template inclusion(levels,inputs) {
    //number of inputs need to be == 2^n
    
    signal input balance[inputs];
    signal input emailHash[inputs];
    signal input sum;
    signal input rootHash;
    signal input userBalance;
    signal input userEmailHash;
    signal output inMerkleTree;
    signal output validMerkleTree;

    signal sumNodes[levels+1][inputs];
    signal hashNodes[levels+1][inputs];
    signal rootHashCalc;
    signal sumCalc;
    signal validHash;
    signal validSum;

    var levelSize = inputs;
    var nextLevelSize = 0;
    var inputsCheck = 0;

    component balanceEqual[inputs];
    component hashEqual[inputs];
    component hashAndBalance[inputs];
    for (var i = 0; i<inputs; i++){
        hashNodes[0][i] <-- emailHash[i];
        sumNodes[0][i] <-- balance[i];

        balanceEqual[i] = IsEqual();
        hashEqual[i] = IsEqual();
        balanceEqual[i].in <== [balance[i],userBalance];
        hashEqual[i].in <== [emailHash[i],userEmailHash];
        hashAndBalance[i] = AND();
        hashAndBalance[i].a <== balanceEqual[i].out;
        hashAndBalance[i].b  <== hashEqual[i].out;

        inputsCheck += hashAndBalance[i].out;     
    }
    inMerkleTree <== inputsCheck;

    component merklesum[levels][inputs];
    for (var i = 0; i < levels; i++) {
        for (var j = 0; j< levelSize; j = j+2)  {
            merklesum[i][j] = MerkleSum();
            
            merklesum[i][j].L <-- hashNodes[i][j];
            merklesum[i][j].R <-- hashNodes[i][j+1];
            merklesum[i][j].sumL <-- sumNodes[i][j];
            merklesum[i][j].sumR <-- sumNodes[i][j+1];

            sumNodes[i+1][nextLevelSize] <-- merklesum[i][j].sum;
            hashNodes[i+1][nextLevelSize] <-- merklesum[i][j].root;
            nextLevelSize +=1;
        }
        levelSize = nextLevelSize;
        nextLevelSize = 0;
    }
    sumCalc <-- sumNodes[levels][0];
    rootHashCalc <-- hashNodes[levels-1][0]; 

    component merkleHashEqual = IsEqual();
    merkleHashEqual.in <== [rootHashCalc,rootHash];
    validHash <== merkleHashEqual.out;

    component sumEqual = IsEqual();
    sumEqual.in <== [sumCalc,sum];
    validSum <== sumEqual.out;

    validMerkleTree <== validHash * validSum;

}

component main = inclusion(2,4);