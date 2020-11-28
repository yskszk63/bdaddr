use aes::{Aes128, BlockCipher, NewBlockCipher};
use generic_array::GenericArray;

use super::Address;

/// Self is not resolvable random address error for [`Address::matches`].
#[derive(Debug, thiserror::Error)]
#[error("invalid address type")]
pub struct InvalidAddressType;

impl Address {
    /// Test matches Identity Resolving Key.
    pub fn matches(&self, irk: &[u8; 16]) -> Result<bool, InvalidAddressType> {
        if (self.0[5] & 0xc0) != 0x40 {
            return Err(InvalidAddressType);
        }

        let k = GenericArray::from_exact_iter(irk.iter().cloned().rev()).unwrap();
        let r = self.0[3..].iter().chain([0; 13].iter()).cloned().rev();
        let r = GenericArray::from_exact_iter(r).unwrap();

        let cipher = Aes128::new(&k);
        let mut hash = r.clone();
        cipher.encrypt_block(&mut hash);
        let hash = &mut hash[13..];
        hash.reverse();

        Ok(hash == &self.0[..3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let addr = Address::from([130, 189, 188, 140, 3, 83]);
        let irk = [
            25, 120, 162, 175, 221, 117, 123, 237, 252, 157, 198, 158, 149, 215, 51, 179,
        ];
        assert!(addr.matches(&irk).unwrap());

        let irk = [
            26, 120, 162, 175, 221, 117, 123, 237, 252, 157, 198, 158, 149, 215, 51, 179,
        ];
        assert!(!addr.matches(&irk).unwrap());
    }
}
