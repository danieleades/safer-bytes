//! A safe, non-panicking wrapper around [`bytes::Buf`]

#![deny(
    clippy::all,
    clippy::cargo,
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)]
#![warn(clippy::pedantic)]

use bytes::Buf;
pub use bytes::BufMut;

pub mod error;
mod safe_buf;

/// Unchecked buffer reading methods
pub mod unchecked {
    pub use bytes::Buf;
}

#[doc(inline)]
pub use error::Error;

/// Type alias for the return type of fallible functions in this crate
pub type Result<T> = std::result::Result<T, Error>;

pub use safe_buf::SafeBuf;

/// Objects which implement [`FromBuf`] are capable of constructing themselves
/// by reading bytes from a [`Buf`]
pub trait FromBuf: Sized {
    /// read an instance of `Self` from a buffer
    ///
    /// # Errors
    ///
    /// This method will return an error if the number of bytes remaining in the
    /// buffer is insufficent, or if the type cannot be parsed from the bytes.
    fn from_buf<B>(buffer: B) -> Result<Self>
    where
        B: Buf;
}
