#![doc(html_root_url = "https://docs.rs/bdaddr/0.1.2")]
#![doc = include_str!("../README.md")]
pub use addr::{
    Address, AddressParseError, NonResolvablePrivateAddress, PublicDeviceAddress,
    RandomDeviceAddress, ResolvablePrivateAddress, StaticDeviceAddress,
};

mod addr;
