#![doc(html_root_url = "https://docs.rs/bdaddr/0.2.0-alpha.1")]
#![doc = include_str!("../README.md")]
pub use addr::{
    Address, AddressParseError, BdAddr, NonResolvablePrivateAddress, PublicDeviceAddress,
    RandomDeviceAddress, ResolvablePrivateAddress, StaticDeviceAddress,
};

mod addr;
