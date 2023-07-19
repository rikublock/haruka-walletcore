// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::coin_context::CoinContext;
use crate::derivation::Derivation;
use crate::error::AddressResult;
use crate::modules::input_builder::InputBuilder;
use crate::modules::json_signer::JsonSigner;
use crate::modules::plan_builder::PlanBuilder;
use crate::prefix::Prefix;
use std::fmt;
use tw_proto::{MessageRead, MessageWrite};

pub use tw_proto::{ProtoError, ProtoResult};

pub type SignatureBytes = Vec<u8>;
pub type PublicKeyBytes = Vec<u8>;

pub trait CoinAddress: fmt::Display {
    fn data(&self) -> Vec<u8>;
}

pub trait CoinEntry {
    type AddressPrefix: Prefix;
    type Address: CoinAddress;
    type SigningInput<'a>: MessageRead<'a> + MessageWrite;
    type SigningOutput: MessageWrite;
    type PreSigningOutput: MessageWrite;

    // Optional modules:
    type JsonSigner: JsonSigner;
    type InputBuilder: InputBuilder<SigningInput = Self::SigningInput<'static>>;
    type PlanBuilder: PlanBuilder;

    /// Tries to parse `Self::Address` from the given `address` string by `coin` type and address `prefix`.
    fn parse_address(
        &self,
        coin: &dyn CoinContext,
        address: &str,
        prefix: Option<Self::AddressPrefix>,
    ) -> AddressResult<Self::Address>;

    /// Derives an address associated with the given `public_key` by `coin` type, `derivation` and address `prefix`.
    fn derive_address(
        &self,
        coin: &dyn CoinContext,
        public_key: PublicKeyBytes,
        derivation: Derivation,
        prefix: Option<Self::AddressPrefix>,
    ) -> AddressResult<Self::Address>;

    /// Signs a transaction declared as the given `input`.
    fn sign(&self, coin: &dyn CoinContext, input: Self::SigningInput<'_>) -> Self::SigningOutput;

    /// Returns hash(es) for signing, needed for external signing.
    fn preimage_hashes(
        &self,
        coin: &dyn CoinContext,
        input: Self::SigningInput<'_>,
    ) -> Self::PreSigningOutput;

    /// Compiles a transaction with externally-supplied `signatures` and `public_keys`.
    fn compile(
        &self,
        coin: &dyn CoinContext,
        input: Self::SigningInput<'_>,
        signatures: Vec<SignatureBytes>,
        public_keys: Vec<PublicKeyBytes>,
    ) -> Self::SigningOutput;

    /// It is optional, Signing JSON input with private key.
    /// Returns `Ok(None)` if the chain doesn't support signing JSON.
    fn json_signer(&self) -> Option<Self::JsonSigner> {
        None
    }

    /// Planning, for UTXO chains, in preparation for signing.
    /// Returns an optional `Plan` builder. Only UTXO chains need it.
    fn plan_builder(&self) -> Option<Self::PlanBuilder> {
        None
    }

    /// Optional helper to prepare a `SigningInput` from simple parameters.
    /// Not suitable for UTXO chains.
    ///
    /// Returns `None` if the chain doesn't support creating `SigningInput` from the simple parameters.
    fn signing_input_builder(&self) -> Option<Self::InputBuilder> {
        None
    }
}
