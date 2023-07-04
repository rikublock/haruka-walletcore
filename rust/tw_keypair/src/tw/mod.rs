// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

mod private;
mod public;

pub use private::PrivateKey;
pub use public::PublicKey;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Curve {
    Secp256k1 = 0,
    Ed25519 = 1,
    Ed25519Blake2bNano = 2,
    /// Waves blockchain specific `curve25519`.
    Curve25519Waves = 3,
    Nist256p1 = 4,
    /// Cardano blockchain specific `ed25519` extended key.
    Ed25519ExtendedCardano = 5,
    Starkex = 6,
}

impl Curve {
    /// Returns `None` if the given curve is not supported in Rust yet.
    pub fn from_raw(curve: u32) -> Option<Curve> {
        match curve {
            0 => Some(Curve::Secp256k1),
            1 => Some(Curve::Ed25519),
            2 => Some(Curve::Ed25519Blake2bNano),
            3 => Some(Curve::Curve25519Waves),
            4 => Some(Curve::Nist256p1),
            5 => Some(Curve::Ed25519ExtendedCardano),
            6 => Some(Curve::Starkex),
            _ => None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum PublicKeyType {
    Secp256k1 = 0,
    Secp256k1Extended = 1,
    Nist256k1 = 2,
    Nist256k1Extended = 3,
    Ed25519 = 4,
    Ed25519Blake2b = 5,
    /// Waves blockchain specific public key.
    Curve25519Waves = 6,
    /// Cardano blockchain specific extended public key.
    Ed25519ExtendedCardano = 7,
    Starkex = 8,
}

impl PublicKeyType {
    /// Returns `None` if the given pubkey type is not supported in Rust yet.
    pub fn from_raw(ty: u32) -> Option<PublicKeyType> {
        match ty {
            0 => Some(PublicKeyType::Secp256k1),
            1 => Some(PublicKeyType::Secp256k1Extended),
            2 => Some(PublicKeyType::Nist256k1),
            3 => Some(PublicKeyType::Nist256k1Extended),
            4 => Some(PublicKeyType::Ed25519),
            5 => Some(PublicKeyType::Ed25519Blake2b),
            6 => Some(PublicKeyType::Curve25519Waves),
            7 => Some(PublicKeyType::Ed25519ExtendedCardano),
            8 => Some(PublicKeyType::Starkex),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_from_raw() {
        let tests = [
            (0, Some(Curve::Secp256k1)),
            (1, Some(Curve::Ed25519)),
            (2, Some(Curve::Ed25519Blake2bNano)),
            (3, Some(Curve::Curve25519Waves)),
            (4, Some(Curve::Nist256p1)),
            (5, Some(Curve::Ed25519ExtendedCardano)),
            (6, Some(Curve::Starkex)),
            (7, None),
        ];
        for (raw, expected) in tests {
            assert_eq!(Curve::from_raw(raw), expected);
        }
    }

    #[test]
    fn test_public_key_type_from_raw() {
        let tests = [
            (0, Some(PublicKeyType::Secp256k1)),
            (1, Some(PublicKeyType::Secp256k1Extended)),
            (2, Some(PublicKeyType::Nist256k1)),
            (3, Some(PublicKeyType::Nist256k1Extended)),
            (4, Some(PublicKeyType::Ed25519)),
            (5, Some(PublicKeyType::Ed25519Blake2b)),
            (6, Some(PublicKeyType::Curve25519Waves)),
            (7, Some(PublicKeyType::Ed25519ExtendedCardano)),
            (8, Some(PublicKeyType::Starkex)),
            (9, None),
        ];
        for (raw, expected) in tests {
            assert_eq!(PublicKeyType::from_raw(raw), expected);
        }
    }
}