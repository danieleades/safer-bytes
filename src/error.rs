//! Internal: Declare an Error type for tor-bytes

/// Error type for decoding Tor objects from bytes.
#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    /// Tried to read something, but not enough bytes left in the buffer
    #[error(transparent)]
    Truncated(#[from] Truncated),

    /// Called Reader::should_be_exhausted(), but found bytes anyway.
    #[error(transparent)]
    ExtraneousBytes(#[from] ExtraneousBytes),

    /// An attempt to parse an object failed for some reason related to its
    /// contents.
    #[error("deserialisation error: {0}")]
    Deserialization(&'static str),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[error("object truncated (or not fully present)")]
pub struct Truncated;

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[error("extra bytes at end of object")]
pub struct ExtraneousBytes;
