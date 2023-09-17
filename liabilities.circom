pragma circom 2.0.0;
include "./merkle.circom";


template createTree(levels,inputs) {
    //prove that we know a merkle tree with a sum of liabilities, where all inputs are <0 and are not bigInt
    //for now we create an unbalanced tree
    
    inputs = 2;
    levels = 2;
    signal input balance[inputs];
    signal input emailHash[inputs];

    signal output sum;
    signal output rootHash;

    //add those 2 checks
    //signal output allPositive;
    //signal output allSmallRange;

    signal sumNodes[levels][inputs];
    signal hashNodes[levels][inputs];

    //hashNodes[0] should be hash of email+balance
    sumNodes[0] <== balance;
    hashNodes[0] <== emailHash;

    var branches = inputs;
    var temp_branches = 0;
    component merklesum[levels][inputs];

    for (var i = 0; i < levels; i++) {
        for (var j = 0; j< branches -1; j = j+2)  {
            merklesum[i][j] = MerkleSumVerifierLevel();
            
            merklesum[i][j].L <== hashNodes[i][j];
            merklesum[i][j].R <== hashNodes[i][j+1];
            merklesum[i][j].sumL <== sumNodes[i][j+1];
            merklesum[i][j].sumR <== sumNodes[i][j+1];

            sumNodes[i+1][j/2] <== merklesum[i][j].sum;
            hashNodes[i+1][j/2] <== merklesum[i][j].root;
            
        temp_branches +=1;
        }
        branches = temp_branches;
        temp_branches = 0;
    } 

    sum <== 1;
    rootHash <== 1;
    //sum <== sumNodes[levels-1][0];
    //rootHash <== hashNodes[levels-1][0]; 

}

component main = createTree(2,2);