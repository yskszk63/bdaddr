use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "matches")]
mod matches;

/// Parse error for [`Address::from_str`]
#[derive(Debug, thiserror::Error)]
#[error("failed to parse address")]
pub struct AddressParseError;

/// Invalid bits for this address type.
#[derive(Debug, thiserror::Error)]
#[error("Invalid bits for this address type. (expect: 0b{0:02b}, but 0b{1:02b})")]
pub struct InvalidBitsForAddressType(u8, u8);

/// BD Address
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BdAddr([u8; 6]);

impl From<[u8; 6]> for BdAddr {
    fn from(v: [u8; 6]) -> Self {
        Self(v)
    }
}

impl From<BdAddr> for [u8; 6] {
    fn from(v: BdAddr) -> Self {
        v.0
    }
}

impl fmt::Display for BdAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[5], self.0[4], self.0[3], self.0[2], self.0[1], self.0[0]
        )
    }
}

impl fmt::Debug for BdAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for BdAddr {
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

impl TryFrom<&str> for BdAddr {
    type Error = AddressParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// LE Public Device Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicDeviceAddress(BdAddr);

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
pub struct NonResolvablePrivateAddress(BdAddr);

impl NonResolvablePrivateAddress {
    const TAG: u8 = 0b00;
}

impl TryFrom<[u8; 6]> for NonResolvablePrivateAddress {
    type Error = InvalidBitsForAddressType;

    fn try_from(v: [u8; 6]) -> Result<Self, Self::Error> {
        if (v[5] & 0xC0) >> 6 == Self::TAG {
            Ok(Self(v.into()))
        } else {
            Err(InvalidBitsForAddressType(Self::TAG, (v[5] & 0xC0) >> 6))
        }
    }
}

impl fmt::Display for NonResolvablePrivateAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// LE Resolvable Private Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvablePrivateAddress(BdAddr);

impl ResolvablePrivateAddress {
    const TAG: u8 = 0b01;
}

impl TryFrom<[u8; 6]> for ResolvablePrivateAddress {
    type Error = InvalidBitsForAddressType;

    fn try_from(v: [u8; 6]) -> Result<Self, Self::Error> {
        if (v[5] & 0xC0) >> 6 == Self::TAG {
            Ok(Self(v.into()))
        } else {
            Err(InvalidBitsForAddressType(Self::TAG, (v[5] & 0xC0) >> 6))
        }
    }
}

impl fmt::Display for ResolvablePrivateAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// LE Static Device Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticDeviceAddress(BdAddr);

impl StaticDeviceAddress {
    const TAG: u8 = 0b11;
}

impl TryFrom<[u8; 6]> for StaticDeviceAddress {
    type Error = InvalidBitsForAddressType;

    fn try_from(v: [u8; 6]) -> Result<Self, Self::Error> {
        if (v[5] & 0xC0) >> 6 == Self::TAG {
            Ok(Self(v.into()))
        } else {
            Err(InvalidBitsForAddressType(Self::TAG, (v[5] & 0xC0) >> 6))
        }
    }
}

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
    Unknown(BdAddr),
}

impl RandomDeviceAddress {
    fn new(addr: BdAddr) -> Self {
        match (addr.0[5] & 0xC0) >> 6 {
            NonResolvablePrivateAddress::TAG => {
                Self::NonResolvable(NonResolvablePrivateAddress(addr))
            }
            ResolvablePrivateAddress::TAG => Self::Resolvable(ResolvablePrivateAddress(addr)),
            StaticDeviceAddress::TAG => Self::Static(StaticDeviceAddress(addr)),
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
    BrEdr(BdAddr),

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
        let addr = BdAddr::from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!("55:44:33:22:11:00", addr.to_string());
    }

    #[test]
    fn test_parse() {
        let addr = "55:44:33:22:11:00".parse().unwrap();
        assert_eq!(BdAddr::from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]), addr);
    }

    #[test]
    fn test_bredr_parse() {
        let addr = Address::bredr_from_str("55:44:33:22:11:00").unwrap();
        assert_eq!("55:44:33:22:11:00", addr.to_string());
        assert!(matches!(addr, Address::BrEdr(..)));

        assert!(Address::bredr_from_str("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ").is_err());
    }

    #[test]
    fn test_le_public_parse() {
        let addr = Address::le_public_from_str("55:44:33:22:11:00").unwrap();
        assert_eq!("55:44:33:22:11:00", addr.to_string());
        assert!(matches!(addr, Address::LePublic(..)));

        assert!(Address::le_public_from_str("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ").is_err());
    }

    #[test]
    fn test_le_random_nonresolvable_parse() {
        let addr = Address::le_random_from_str("35:44:33:22:11:00").unwrap();
        assert_eq!("35:44:33:22:11:00", addr.to_string());
        assert!(matches!(
            addr,
            Address::LeRandom(RandomDeviceAddress::NonResolvable(..))
        ));

        assert!(Address::le_random_from_str("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ").is_err());
    }

    #[test]
    fn test_le_random_resolvable_parse() {
        let addr = Address::le_random_from_str("75:44:33:22:11:00").unwrap();
        assert_eq!("75:44:33:22:11:00", addr.to_string());
        assert!(matches!(
            addr,
            Address::LeRandom(RandomDeviceAddress::Resolvable(..))
        ));

        assert!(Address::le_random_from_str("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ").is_err());
    }

    #[test]
    fn test_le_random_static_parse() {
        let addr = Address::le_random_from_str("F5:44:33:22:11:00").unwrap();
        assert_eq!("f5:44:33:22:11:00", addr.to_string());
        assert!(matches!(
            addr,
            Address::LeRandom(RandomDeviceAddress::Static(..))
        ));

        assert!(Address::le_random_from_str("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ").is_err());
    }

    #[test]
    fn test_le_random_unknown_parse() {
        let addr = Address::le_random_from_str("B5:44:33:22:11:00").unwrap();
        assert_eq!("b5:44:33:22:11:00", addr.to_string());
        assert!(matches!(
            addr,
            Address::LeRandom(RandomDeviceAddress::Unknown(..))
        ));

        assert!(Address::le_random_from_str("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ").is_err());
    }

    #[test]
    fn test_bredr_from() {
        let addr = Address::bredr_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(matches!(addr, Address::BrEdr(..)));
    }

    #[test]
    fn test_le_public_from() {
        let addr = Address::le_public_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(matches!(addr, Address::LePublic(..)));
    }

    #[test]
    fn test_le_random_from() {
        let addr = Address::le_random_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(matches!(addr, Address::LeRandom(..)));
    }

    #[test]
    fn test_non_resolvable_try_from() {
        let addr = NonResolvablePrivateAddress::try_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x35]);
        assert!(matches!(addr, Ok(NonResolvablePrivateAddress(..))));

        let addr = NonResolvablePrivateAddress::try_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(matches!(addr, Err(InvalidBitsForAddressType(0b00, 0b01))));
    }

    #[test]
    fn test_resolvable_try_from() {
        let addr = ResolvablePrivateAddress::try_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(matches!(addr, Ok(ResolvablePrivateAddress(..))));

        let addr = ResolvablePrivateAddress::try_from([0x00, 0x11, 0x22, 0x33, 0x44, 0xB5]);
        assert!(matches!(addr, Err(InvalidBitsForAddressType(0b01, 0b10))));
    }

    #[test]
    fn test_static_try_from() {
        let addr = StaticDeviceAddress::try_from([0x00, 0x11, 0x22, 0x33, 0x44, 0xF5]);
        assert!(matches!(addr, Ok(StaticDeviceAddress(..))));

        let addr = StaticDeviceAddress::try_from([0x00, 0x11, 0x22, 0x33, 0x44, 0x05]);
        println!("{:?}", addr);
        assert!(matches!(addr, Err(InvalidBitsForAddressType(0b11, 0b00))));
    }
}
