const buildMimcSponge = require("circomlibjs").buildMimcSponge;

class MerkleTree {
  async getMimic() {
    this.mimcSponge = await buildMimcSponge();
  }

  async getHash(left_hash, right_hash, left_sum, right_sum) {
    let F = this.mimcSponge.F;

    const out2 = this.mimcSponge.multiHash(
      [left_hash, right_hash, left_sum, right_sum],
      0,
      3
    );

    for (let i = 0; i < out2.length; i++) out2[i] = F.toObject(out2[i]);

    return [out2[0], left_sum + right_sum];
  }

  async getRoot(balance, emailHash, levels, inputs) {
    let sumNodes = [[], [], []];
    let hashNodes = [[], [], []];
    let levelSize = inputs;
    let nextLevelSize = 0;

    for (var i = 0; i < inputs; i++) {
      hashNodes[0][i] = emailHash[i];
      sumNodes[0][i] = balance[i];
    }

    for (var i = 0; i < levels; i++) {
      for (var j = 0; j < levelSize; j = j + 2) {
        let values = await this.getHash(
          hashNodes[i][j],
          hashNodes[i][j + 1],
          sumNodes[i][j],
          sumNodes[i][j + 1]
        );
        hashNodes[i + 1][nextLevelSize] = values[0];
        sumNodes[i + 1][nextLevelSize] = values[1];
        nextLevelSize = nextLevelSize + 1;
      }
      levelSize = nextLevelSize;
      nextLevelSize = 0;
    }
    console.log(hashNodes)
    return [hashNodes[levels][0], sumNodes[levels][0]];
  }
}
async function main(){
    const merkleTree = new MerkleTree();
    await merkleTree.getMimic();
    let outputs = await merkleTree.getRoot(
      [10, 11, 12, 13],
      [11672136, 10566265, 3423253245, 5342523],
      2,
      4
    );
    console.log(outputs[0]);
    console.log(outputs[1]);
}

main()