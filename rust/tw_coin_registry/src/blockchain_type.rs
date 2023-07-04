// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::RegistryError;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// Blockchain implementation type.
/// TODO extend this enum.
#[derive(Copy, Clone, Debug)]
pub enum BlockchainType {
    Ethereum,
    Unsupported,
}

impl<'de> Deserialize<'de> for BlockchainType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BlockchainType::from_str(&s).map_err(|e| Error::custom(format!("{e:?}")))
    }
}

impl FromStr for BlockchainType {
    type Err = RegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ethereum" => Ok(BlockchainType::Ethereum),
            _ => Ok(BlockchainType::Unsupported),
        }
    }
}