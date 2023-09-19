pragma circom 2.0.0;
include "./node_modules/circomlib/circuits/comparators.circom";

template NegativeCheck(maxBits){
    signal input in;
    signal output out;

    component lessthan = LessThan(maxBits);
    lessthan.in[0] <== -1;
    lessthan.in[1] <== in;
    out <== lessthan.out;

}

template RangeCheck(){

}