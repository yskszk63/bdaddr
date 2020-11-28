#![doc(html_root_url = "https://docs.rs/bdaddr/0.1.0")]
//! Bluetooth Device Address.
//!
//! ## Dependencies
//!
//! ```toml
//! [dependencies]
//! bdaddr = "0.1"
//! ```
//!
//! ## Example
//!
//! ```
//! use bdaddr::Address;
//!
//! fn main() {
//!     let addr = "00:11:22:33:44:55".parse::<Address>().unwrap();
//!     assert_eq!(addr.to_string(), "00:11:22:33:44:55");
//! }
//! ```
//!
//! ## License
//!
//! Licensed under either of
//! * Apache License, Version 2.0
//!   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.!
#[cfg(feature = "matches")]
pub use addr::InvalidAddressType;
pub use addr::{Address, AddressParseError};

mod addr;
