use std::convert::TryInto;

pub const BITS_PER_BYTE: usize = 8;
pub const MAX_UVARINT_NUM_BYTES: usize = 9;

#[derive(Debug)]
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

    pub fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let num_bytes: usize = (UVarInt::u128_log2(self.num) /
            (BITS_PER_BYTE - 1)) + 1;
        
        /* bounds check the number of bytes produced */
        if num_bytes > MAX_UVARINT_NUM_BYTES {
            return Err(EncodeError::OutOfRange);
        }

        /* define the bytes structure we'll use to assemble binary layout */
        let mut bytes: Vec<u8> = vec![0u8; num_bytes];

        /* base case where number fits entirely into one byte */
        if self.num <= std::i8::MAX as u128 {
            bytes = vec![self.num.try_into().unwrap()];
            return Ok(bytes);
        }

        /* encode byte-at-a-time */
        let mut n: u128 = self.num;

        for i in 0..num_bytes {
            bytes[i] = (n | 0x80) as u8;
            n >>= 7;

            if i + 1 == num_bytes {
                bytes[i] &= 0x7f;
                break;
            }
        }

        Ok(bytes)
    }

    fn u128_log2(n: u128) -> usize {
        (std::mem::size_of::<u128>() * BITS_PER_BYTE) -
            n.leading_zeros() as usize - 1
    }
}

