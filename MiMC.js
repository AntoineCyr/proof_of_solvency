/* global BigInt */
const circomlibjs = require("circomlibjs");


class MimcSpongeHasher {
  hash(level, left, right) {
    circomlibjs.buildMimcSponge().then((mimicsponge) => {
        console.log("here2")
      return mimicsponge.multiHash([bigInt(left), bigInt(right)],0,1)[0].toString();
    });
  }
}

module.exports = MimcSpongeHasher;
