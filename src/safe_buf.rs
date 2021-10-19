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
    /// Peek at a given number of bytes from the buffer, with a check to ensure
    /// there are enough remaining
    ///
    /// Use this version when you know the array length at compile time.
    /// Otherwise use [`SafeBuf::try_peek`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent
    fn try_peek_const<const N: usize>(&mut self) -> Result<[u8; N], error::Truncated> {
        if self.remaining() < N {
            Err(error::Truncated)
        } else {
            let mut bytes = [0_u8; N];
            self.copy_to_slice(&mut bytes);
            Ok(bytes)
        }
    }

    /// Take a given number of bytes from the buffer, with a check to ensure
    /// there are enough remaining
    ///
    /// Use this version when you know the array length at compile time.
    /// Otherwise use [`SafeBuf::try_take`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent
    fn try_take_const<const N: usize>(&mut self) -> Result<[u8; N], error::Truncated> {
        let bytes = self.try_peek_const()?;
        self.advance(N);
        Ok(bytes)
    }

    /// Peek at a given number of bytes from the buffer, with a check to ensure
    /// there are enough remaining
    ///
    /// If you know the array length at compile time, use
    /// [`SafeBuf::try_peek_const`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent
    fn try_peek(&mut self, len: usize) -> std::result::Result<Bytes, error::Truncated> {
        if self.remaining() < len {
            Err(error::Truncated)
        } else {
            Ok(self.copy_to_bytes(len))
        }
    }

    /// Take a given number of bytes from the buffer, with a check to ensure
    /// there are enough remaining
    ///
    /// If you know the array length at compile time, use
    /// [`SafeBuf::try_take_const`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent
    fn try_take(&mut self, len: usize) -> std::result::Result<Bytes, error::Truncated> {
        let bytes = self.try_peek(len)?;
        self.advance(len);
        Ok(bytes)
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
