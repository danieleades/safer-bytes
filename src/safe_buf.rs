//! Extension traits for extracting custom objects from a [`bytes::Buf`]

use crate::{error, FromBuf};
use bytes::{Buf, Bytes};
use paste::paste;

macro_rules! get_primitive_checked_be {
    ($t:ty, $width:literal) => {
        paste! {
            #[doc = "This method wraps [`Buf::get_" $t "`] with a bounds check to ensure there are enough bytes remaining, without panicking."]
            fn [<try_get_ $t>](&mut self) -> std::result::Result<$t, error::Truncated> {
                if self.remaining() >= $width {
                    Ok(self.[<get_ $t>]())
                } else {
                    Err(error::Truncated)
                }
            }
        }
    };
}

macro_rules! get_primitive_checked_le {
    ($t:ty, $width:literal) => {
        paste! {
            #[doc = "This method wraps [`Buf::get_" $t "_le`] with a bounds check to ensure there are enough bytes remaining, without panicking."]
            fn [<try_get_ $t _le>](&mut self) -> std::result::Result<$t, error::Truncated> {
                if self.remaining() >= $width {
                    Ok(self.[<get_ $t _le>]())
                } else {
                    Err(error::Truncated)
                }
            }
        }
    };
}

/// Extension trait for [`bytes::Buf`]
pub trait SafeBuf: Buf {
    /// Take a given number of bytes from the buffer, with a check to ensure
    /// there are enough remaining
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent
    fn try_copy_to_bytes(&mut self, len: usize) -> std::result::Result<Bytes, error::Truncated> {
        if self.remaining() < len {
            Err(error::Truncated)
        } else {
            Ok(self.copy_to_bytes(len))
        }
    }

    /// Take a given number of bytes from the buffer and write to a slice, with
    /// a check to ensure there are enough remaining
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent
    fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> std::result::Result<(), error::Truncated> {
        if self.remaining() < dst.len() {
            Err(error::Truncated)
        } else {
            self.copy_to_slice(dst);
            Ok(())
        }
    }

    /// Read a custom object from a buffer
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent, or if the type cannot be parsed from the bytes.
    fn extract<T>(&mut self) -> crate::Result<T>
    where
        T: FromBuf,
    {
        T::from_buf(self)
    }

    /// Check whether this reader is exhausted (out of bytes).
    ///
    /// # Errors
    ///
    /// this method will return [`error::ExtraneousBytes`] if there are bytes
    /// left in the buffer.
    fn should_be_exhausted(&self) -> std::result::Result<(), error::ExtraneousBytes> {
        if self.has_remaining() {
            Err(error::ExtraneousBytes)
        } else {
            Ok(())
        }
    }

    get_primitive_checked_be!(u8, 1);
    get_primitive_checked_be!(i8, 1);

    get_primitive_checked_be!(u16, 2);
    get_primitive_checked_be!(i16, 2);
    get_primitive_checked_be!(u32, 4);
    get_primitive_checked_be!(i32, 4);
    get_primitive_checked_be!(u64, 8);
    get_primitive_checked_be!(i64, 8);
    get_primitive_checked_be!(u128, 16);
    get_primitive_checked_be!(i128, 16);

    get_primitive_checked_le!(u16, 2);
    get_primitive_checked_le!(i16, 2);
    get_primitive_checked_le!(u32, 4);
    get_primitive_checked_le!(i32, 4);
    get_primitive_checked_le!(u64, 8);
    get_primitive_checked_le!(i64, 8);
    get_primitive_checked_le!(u128, 16);
    get_primitive_checked_le!(i128, 16);
}

impl<T> SafeBuf for T where T: Buf {}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use paste::paste;

    use super::SafeBuf;
    use crate::BufMut;

    #[test]
    fn try_copy_to_bytes() {
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert!(bytes.try_copy_to_bytes(4).is_ok());
        assert!(bytes.try_copy_to_bytes(4).is_ok());
        assert!(bytes.try_copy_to_bytes(4).is_err());
    }

    #[test]
    fn try_copy_to_slice() {
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let dst = &mut [0_u8; 4];

        assert!(bytes.try_copy_to_slice(dst).is_ok());
        assert!(bytes.try_copy_to_slice(dst).is_ok());
        assert!(bytes.try_copy_to_slice(dst).is_err());
    }

    macro_rules! round_trip {
        ($t:ty) => {
            paste! {
                #[test]
                fn [<round_trip_ $t>]() {
                    let mut buffer = BytesMut::new();
                    let input = 17;

                    buffer.[<put_ $t>](input);
                    let output = buffer.[<try_get_ $t>]().unwrap();

                    assert!(buffer.[<try_get_ $t>]().is_err());
                    assert_eq!(input, output);
                    assert!(buffer.is_empty());
                }
            }
        };
    }

    round_trip!(u8);
    round_trip!(i8);
    round_trip!(u16);
    round_trip!(i16);
    round_trip!(u32);
    round_trip!(i32);
    round_trip!(u64);
    round_trip!(i64);
}
