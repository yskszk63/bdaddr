#![doc(html_root_url = "https://docs.rs/bdaddr/0.2.0-alpha.4")]
#![doc = include_str!("../README.md")]
pub use addr::{
    Address, AddressParseError, AddressType, BdAddr, NonResolvablePrivateAddress,
    PublicDeviceAddress, RandomDeviceAddress, ResolvablePrivateAddress, StaticDeviceAddress,
};

mod addr;
