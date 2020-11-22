pub use addr::{Address, AddressParseError};
#[cfg(feature = "matches")]
pub use addr::InvalidAddressType;

mod addr;
