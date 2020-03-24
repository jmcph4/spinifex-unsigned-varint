use std::convert::TryInto;

pub const BITS_PER_BYTE: usize = 8;
pub const MAX_UVARINT_NUM_BYTES: usize = 9;

pub enum EncodeError {
    OutOfRange
}

pub struct UVarInt {
    num: u128
}

impl UVarInt {
    pub fn new(num: u128) -> Self {
        UVarInt {
            num: num
        }
    }

}

