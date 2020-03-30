use std::fmt;
use std::convert::TryInto;

/// Number of bits in a byte.
///
/// This type largely exists to avoid magic numbers littering the codebase.
pub const BITS_PER_BYTE: usize = 8;

/// Maximum number of bytes in a binary representation for a `UVarInt`.
///
/// See the multiformat specification for details.
pub const MAX_UVARINT_NUM_BYTES: usize = 9;

/// Represents an encoding failure.
///
/// Returned whenever a function performs encoding of a `UVarInt` type.
#[derive(Debug)]
pub enum EncodeError {
    OutOfRange
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodeError::OutOfRange => 
                write!(f, "Value overflows maximum output size")?
        };

        Ok(())
    }
}

/// Represents a decoding failure.
///
/// Returned whenever a function performs decoding of a `UVarInt` type.
#[derive(Debug)]
pub enum DecodeError {
    OutOfRange
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::OutOfRange => 
                write!(f, "Input size overflows native representation")?
        };

        Ok(())
    }
}

/// Represents an unsigned variable integer type, compliant with the multiformat
/// of the same name.
///
/// The struct simply contains the underlying native integer type representing
/// the type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Default, Hash)]
pub struct UVarInt {
    num: u128
}

impl UVarInt {
    /// Constructs a new `UVarInt` from a native unsigned integer type.
    ///
    /// # Examples #
    /// ```rust
    /// use spinifex_unsigned_varint::uvarint::UVarInt;
    ///
    /// fn main() {
    ///     let some_uvarint: UVarInt = UVarInt::new(128);
    ///     println!("{}", some_uvarint);
    /// }
    ///
    /// ```
    pub fn new(num: u128) -> Self {
        UVarInt {
            num: num
        }
    }

    /// Encodes the `UVarInt` type into its binary representation (as a
    /// `Vec<u8>`).
    ///
    /// # Examples #
    /// ```rust
    /// use spinifex_unsigned_varint::uvarint::{UVarInt, EncodeError};
    ///
    /// fn main() {
    ///     let bytes: Vec<u8> = vec![128, 1];
    ///     let some_uvarint: UVarInt = match UVarInt::from_bytes(bytes) {
    ///         Ok(uv) => uv,
    ///         Err(e) => {
    ///             println!("{:?}", e);
    ///             panic!();
    ///         }
    ///     };
    /// 
    ///     println!("Bytes decoded as {}", some_uvarint);
    /// }
    ///
    /// ```
    /// 
    /// # Errors #
    /// 
    /// Returns `EncodeError::OutOfRange` if the stored value would overflow the
    /// maximum number of bytes of an unsigned varint (`MAX_UVARINT_NUM_BYTES`).
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

    /// Decodes a sequence of bytes (as a `Vec<u8>`) into a valid `UVarInt`.
    ///
    /// # Examples #
    /// 
    /// ```rust
    /// use spinifex_unsigned_varint::uvarint::{UVarInt, EncodeError};
    ///
    /// fn main() {
    ///     let some_uvarint: UVarInt = UVarInt::new(128);
    ///     let bytes: Vec<u8> = match some_uvarint.to_bytes() {
    ///         Ok(b) => b,
    ///         Err(e) => {
    ///             println!("{:?}", e);
    ///             panic!();
    ///         }
    ///     };
    /// 
    ///     println!("UVarInt encoded to {:?}", bytes);
    /// }
    ///
    /// ```
    ///
    /// # Errors #
    /// 
    /// Returns `DecodeError::OutOfRange` if the number of provided bytes
    /// exceeds `MAX_UVARINT_NUM_BYTES`.
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

    /// Calculates the (floor of the) base 2 logarithm of a native 128-bit
    /// unsigned integer.
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

