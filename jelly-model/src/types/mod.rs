mod chain;
mod identity;
mod time;

pub use time::TimestampMills;

/// HASH value
pub type ContentHash = [u8; 32];

pub use identity::{HashIdentity, StringIdentity, U64Identity};

/// The maximum support data length is recommended to store separately
#[allow(unused)]
pub(crate) const MAX_DATA_LENGTH: usize = 256;

pub use chain::CallChain;
