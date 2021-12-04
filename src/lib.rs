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
pub use bytes::{BufMut, Bytes, BytesMut};

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
