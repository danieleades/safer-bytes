//! Errors

/// Errors that can occur when deserialising objects from a buffer
#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    /// Tried to read something, but not enough bytes left in the buffer
    #[error(transparent)]
    Truncated(#[from] Truncated),

    /// Called Reader::should_be_exhausted(), but found bytes anyway.
    #[error(transparent)]
    ExtraneousBytes(#[from] ExtraneousBytes),
}

/// Tried to read something, but not enough bytes left in the buffer
#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[error("object truncated (or not fully present)")]
pub struct Truncated;

#[rustfmt::skip]
/// Called [`SafeBuf::should_be_exhausted`](crate::SafeBuf::should_be_exhausted), but found bytes remaining
#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[error("extra bytes at end of object")]
pub struct ExtraneousBytes;
