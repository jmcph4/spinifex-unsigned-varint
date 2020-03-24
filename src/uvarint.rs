use std::fmt;
use std::convert::TryInto;

pub const BITS_PER_BYTE: usize = 8;
pub const MAX_UVARINT_NUM_BYTES: usize = 9;

#[derive(Debug)]
pub enum EncodeError {
    OutOfRange
}

#[derive(Debug)]
pub enum DecodeError {
    OutOfRange
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
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

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, DecodeError> {
        if bytes.len() > MAX_UVARINT_NUM_BYTES { /* bounds check */
            return Err(DecodeError::OutOfRange);
        }

        let mut num: u128 = 0;

        let mut n: u128 = 0;
        let mut k: u128 = 0;

        for i in 0..bytes.len() {
            k = (bytes[i] & 0x7f) as u128;
            n |= k << (i * 7);

            if (bytes[i] & 0x80) == 0 {
                num = n;
                break;
            }
        }

        let varint: UVarInt = UVarInt::new(num);
        Ok(varint)
    }

    fn u128_log2(n: u128) -> usize {
        (std::mem::size_of::<u128>() * BITS_PER_BYTE) -
            n.leading_zeros() as usize - 1
    }
}

impl fmt::Display for UVarInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uv{}", self.num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes_spec1() -> Result<(), EncodeError> {
        let number: u128 = 1;
        let actual_uvarint: UVarInt = UVarInt::new(number);
        
        let actual_bytes: Vec<u8> = actual_uvarint.to_bytes()?;
        let expected_bytes: Vec<u8> = vec![1];

        assert_eq!(actual_bytes, expected_bytes);
        Ok(())
    }

    #[test]
    fn test_to_bytes_spec2() -> Result<(), EncodeError> {
        let number: u128 = 127;
        let actual_uvarint: UVarInt = UVarInt::new(number);
        
        let actual_bytes: Vec<u8> = actual_uvarint.to_bytes()?;
        let expected_bytes: Vec<u8> = vec![127];

        assert_eq!(actual_bytes, expected_bytes);
        Ok(())
    }

    #[test]
    fn test_to_bytes_spec3() -> Result<(), EncodeError> {
        let number: u128 = 128;
        let actual_uvarint: UVarInt = UVarInt::new(number);
        
        let actual_bytes: Vec<u8> = actual_uvarint.to_bytes()?;
        let expected_bytes: Vec<u8> = vec![128, 1];

        assert_eq!(actual_bytes, expected_bytes);
        Ok(())
    }

    #[test]
    fn test_to_bytes_spec4() -> Result<(), EncodeError> {
        let number: u128 = 255;
        let actual_uvarint: UVarInt = UVarInt::new(number);
        
        let actual_bytes: Vec<u8> = actual_uvarint.to_bytes()?;
        let expected_bytes: Vec<u8> = vec![255, 1];

        assert_eq!(actual_bytes, expected_bytes);
        Ok(())
    }

    #[test]
    fn test_to_bytes_spec5() -> Result<(), EncodeError> {
        let number: u128 = 300;
        let actual_uvarint: UVarInt = UVarInt::new(number);
        
        let actual_bytes: Vec<u8> = actual_uvarint.to_bytes()?;
        let expected_bytes: Vec<u8> = vec![172, 2];

        assert_eq!(actual_bytes, expected_bytes);
        Ok(())
    }

    #[test]
    fn test_to_bytes_spec6() -> Result<(), EncodeError> {
        let number: u128 = 16384;
        let actual_uvarint: UVarInt = UVarInt::new(number);
        
        let actual_bytes: Vec<u8> = actual_uvarint.to_bytes()?;
        let expected_bytes: Vec<u8> = vec![128, 128, 1];

        assert_eq!(actual_bytes, expected_bytes);
        Ok(())
    }

    #[test]
    fn test_from_bytes_spec1() -> Result<(), DecodeError> {
        let number: u128 = 1;
        let bytes: Vec<u8> = vec![1];

        let actual_uvarint: UVarInt = UVarInt::from_bytes(bytes)?;
        let expected_uvarint: UVarInt = UVarInt::new(number);

        assert_eq!(actual_uvarint, expected_uvarint);
        Ok(())
    }
    
    #[test]
    fn test_from_bytes_spec2() -> Result<(), DecodeError> {
        let number: u128 = 127;
        let bytes: Vec<u8> = vec![127];

        let actual_uvarint: UVarInt = UVarInt::from_bytes(bytes)?;
        let expected_uvarint: UVarInt = UVarInt::new(number);

        assert_eq!(actual_uvarint, expected_uvarint);
        Ok(())
    }

    #[test]
    fn test_from_bytes_spec3() -> Result<(), DecodeError> {
        let number: u128 = 128;
        let bytes: Vec<u8> = vec![128, 1];

        let actual_uvarint: UVarInt = UVarInt::from_bytes(bytes)?;
        let expected_uvarint: UVarInt = UVarInt::new(number);

        assert_eq!(actual_uvarint, expected_uvarint);
        Ok(())
    }

    #[test]
    fn test_from_bytes_spec4() -> Result<(), DecodeError> {
        let number: u128 = 255;
        let bytes: Vec<u8> = vec![255, 1];

        let actual_uvarint: UVarInt = UVarInt::from_bytes(bytes)?;
        let expected_uvarint: UVarInt = UVarInt::new(number);

        assert_eq!(actual_uvarint, expected_uvarint);
        Ok(())
    }

    #[test]
    fn test_from_bytes_spec5() -> Result<(), DecodeError> {
        let number: u128 = 300;
        let bytes: Vec<u8> = vec![172, 2];

        let actual_uvarint: UVarInt = UVarInt::from_bytes(bytes)?;
        let expected_uvarint: UVarInt = UVarInt::new(number);

        assert_eq!(actual_uvarint, expected_uvarint);
        Ok(())
    }

    #[test]
    fn test_from_bytes_spec6() -> Result<(), DecodeError> {
        let number: u128 = 16384;
        let bytes: Vec<u8> = vec![128, 128, 1];

        let actual_uvarint: UVarInt = UVarInt::from_bytes(bytes)?;
        let expected_uvarint: UVarInt = UVarInt::new(number);

        assert_eq!(actual_uvarint, expected_uvarint);
        Ok(())
    }
}

