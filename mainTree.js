const MerkleTree = require("./merkleTree")
const n_levels = 2
const zero_value =1 
const defaultElements = [10,11,12,13]
const prefix = null
const merkleTree = new MerkleTree(n_levels, zero_value, defaultElements, prefix)