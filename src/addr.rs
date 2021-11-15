use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "matches")]
mod matches;

/// Parse error for [`Address::from_str`]
#[derive(Debug, thiserror::Error)]
#[error("failed to parse address")]
pub struct AddressParseError;

/// BD Address
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RawAddress([u8; 6]);

impl From<[u8; 6]> for RawAddress {
    fn from(v: [u8; 6]) -> Self {
        Self(v)
    }
}

impl From<RawAddress> for [u8; 6] {
    fn from(v: RawAddress) -> Self {
        v.0
    }
}

impl fmt::Display for RawAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[5], self.0[4], self.0[3], self.0[2], self.0[1], self.0[0]
        )
    }
}

impl fmt::Debug for RawAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for RawAddress {
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

impl TryFrom<&str> for RawAddress {
    type Error = AddressParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// LE Public Device Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicDeviceAddress(RawAddress);

impl From<[u8; 6]> for PublicDeviceAddress {
    fn from(v: [u8; 6]) -> Self {
        Self(v.into())
    }
}

impl fmt::Display for PublicDeviceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// LE Non-Resolvable Private Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonResolvablePrivateAddress(RawAddress);

impl fmt::Display for NonResolvablePrivateAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// LE Resolvable Private Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvablePrivateAddress(RawAddress);

impl fmt::Display for ResolvablePrivateAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// LE Static Device Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticDeviceAddress(RawAddress);

impl fmt::Display for StaticDeviceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// LE Random Device Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RandomDeviceAddress {
    /// Non-Resolvable Private Address
    NonResolvable(NonResolvablePrivateAddress),

    /// Resolvable Private Address
    Resolvable(ResolvablePrivateAddress),

    /// Static Device Address
    Static(StaticDeviceAddress),

    /// Unknown
    Unknown(RawAddress),
}

impl RandomDeviceAddress {
    fn new(addr: RawAddress) -> Self {
        match (addr.0[5] & 0xC0) >> 6 {
            0b00 => Self::NonResolvable(NonResolvablePrivateAddress(addr)),
            0b10 => Self::Resolvable(ResolvablePrivateAddress(addr)),
            0b11 => Self::Static(StaticDeviceAddress(addr)),
            _ => Self::Unknown(addr),
        }
    }
}

impl From<[u8; 6]> for RandomDeviceAddress {
    fn from(v: [u8; 6]) -> Self {
        Self::new(v.into())
    }
}

impl fmt::Display for RandomDeviceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonResolvable(v) => v.fmt(f),
            Self::Resolvable(v) => v.fmt(f),
            Self::Static(v) => v.fmt(f),
            Self::Unknown(v) => v.fmt(f),
        }
    }
}

/// Bluetooth Device Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Address {
    /// Classic BR/EDR Address
    BrEdr(RawAddress),

    /// LE Public Device Address
    LePublic(PublicDeviceAddress),

    /// LE Random Device Address
    LeRandom(RandomDeviceAddress),
}

impl Address {
    /// Construct Classic BR/EDR Address from bytes.
    pub fn bredr_from(b: [u8; 6]) -> Self {
        Self::BrEdr(b.into())
    }

    /// Construct LE Public Device Address from bytes.
    pub fn le_public_from(b: [u8; 6]) -> Self {
        Self::LePublic(b.into())
    }

    /// Construct LE Random Device Address from bytes.
    pub fn le_random_from(b: [u8; 6]) -> Self {
        Self::LeRandom(b.into())
    }

    /// Construct Classic BR/EDR Address from str.
    pub fn bredr_from_str(s: &str) -> Result<Self, AddressParseError> {
        Ok(Self::BrEdr(s.parse()?))
    }

    /// Construct LE Public Device Address from str.
    pub fn le_public_from_str(s: &str) -> Result<Self, AddressParseError> {
        Ok(Self::LePublic(PublicDeviceAddress(s.parse()?)))
    }

    /// Construct LE Public Device Address from str.
    pub fn le_random_from_str(s: &str) -> Result<Self, AddressParseError> {
        Ok(Self::LeRandom(RandomDeviceAddress::new(s.parse()?)))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrEdr(v) => v.fmt(f),
            Self::LePublic(v) => v.fmt(f),
            Self::LeRandom(v) => v.fmt(f),
        }
    }
}
// TODO memo BLUETOOTH CORE SPECIFICATION | Vol 6, Part B | 1.3 DEVICE ADDRESS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let addr = RawAddress::from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!("55:44:33:22:11:00", addr.to_string());
    }

    #[test]
    fn test_parse() {
        let addr = "55:44:33:22:11:00".parse().unwrap();
        assert_eq!(RawAddress::from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]), addr);
    }
}
