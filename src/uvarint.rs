#![allow(dead_code)]
extern crate bitvec;
use bitvec::vec::BitVec;

pub struct UVarInt {
    num: u128,
    bits: BitVec
}

