use aes::cipher::generic_array::GenericArray;
use aes::{Aes128, BlockEncrypt, NewBlockCipher};

use super::ResolvablePrivateAddress;

impl ResolvablePrivateAddress {
    /// Test matches Identity Resolving Key.
    pub fn matches(&self, irk: &[u8; 16]) -> bool {
        let k = GenericArray::from_exact_iter(irk.iter().cloned().rev()).unwrap();
        let r = self.0 .0[3..].iter().chain([0; 13].iter()).cloned().rev();
        let r = GenericArray::from_exact_iter(r).unwrap();

        let cipher = Aes128::new(&k);
        let mut hash = r.clone();
        cipher.encrypt_block(&mut hash);
        let hash = &mut hash[13..];
        hash.reverse();

        hash == &self.0 .0[..3] // TODO Not sure if this is a good way to compare.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // TODO Construct RPA from bytes.
        let addr = ResolvablePrivateAddress::from([130, 189, 188, 140, 3, 83]);
        let irk = [
            25, 120, 162, 175, 221, 117, 123, 237, 252, 157, 198, 158, 149, 215, 51, 179,
        ];
        assert!(addr.matches(&irk));

        let irk = [
            26, 120, 162, 175, 221, 117, 123, 237, 252, 157, 198, 158, 149, 215, 51, 179,
        ];
        assert!(!addr.matches(&irk));
    }
}
