// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

/// TODO extend this enum.
/// TODO make this trait.
pub enum Derivation {
    /// Default derivation.
    Default = 0,
}

impl Derivation {
    pub fn from_raw(derivation: u32) -> Option<Derivation> {
        match derivation {
            0 => Some(Derivation::Default),
            _ => None,
        }
    }
}
