use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "matches")]
pub use matches::InvalidAddressType;

#[cfg(feature = "matches")]
mod matches;

/// Parse error for [`Address::from_str`]
#[derive(Debug, thiserror::Error)]
#[error("failed to parse address")]
pub struct AddressParseError;

/// Sub-types of random device address.
///
/// BLUETOOTH CORE SPECIFICATION | Vol 6, Part B | 1.3.2 Random device address - Table 1.2
pub enum SubType {
    /// Non-Resolvable private address
    NonResolvable,

    /// Resolvable private address
    Resolvable,

    /// Reserved for future use
    Reserved,

    /// Static device address
    Static,
}

/// Bluetooth Device Address
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Address([u8; 6]);

impl Address {
    /// Get Sub-types of random device address, If this address is random address.
    pub fn sub_type(&self) -> SubType {
        // TODO test
        match (self.0[5] & 0xc0) >> 6 {
            0x00 => SubType::NonResolvable,
            0x01 => SubType::Resolvable,
            0x04 => SubType::Static,
            _ => SubType::Reserved,
        }
    }
}

impl From<[u8; 6]> for Address {
    fn from(v: [u8; 6]) -> Self {
        Self(v)
    }
}

impl From<Address> for [u8; 6] {
    fn from(v: Address) -> Self {
        v.0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[5], self.0[4], self.0[3], self.0[2], self.0[1], self.0[0]
        )
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Address {
    type Err = AddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .splitn(6, ':')
            .map(|v| u8::from_str_radix(v, 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| AddressParseError)?;
        parts.reverse();
        Ok(Self(parts.try_into().map_err(|_| AddressParseError)?))
    }
}

impl TryFrom<&str> for Address {
    type Error = AddressParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let addr = Address::from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!("55:44:33:22:11:00", addr.to_string());
    }

    #[test]
    fn test_parse() {
        let addr = "55:44:33:22:11:00".parse().unwrap();
        assert_eq!(Address::from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]), addr);
    }
}
