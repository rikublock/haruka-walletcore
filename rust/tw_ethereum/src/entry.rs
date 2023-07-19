// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::address::Address;
use crate::modules::compiler::Compiler;
use crate::modules::input_builder::EthInputBuilder;
use crate::modules::json_signer::EthJsonSigner;
use crate::modules::signer::Signer;
use std::str::FromStr;
use tw_coin_entry::coin_context::CoinContext;
use tw_coin_entry::coin_entry::{CoinEntry, PublicKeyBytes, SignatureBytes};
use tw_coin_entry::derivation::Derivation;
use tw_coin_entry::error::{AddressError, AddressResult};
use tw_coin_entry::modules::plan_builder::NoPlanBuilder;
use tw_coin_entry::prefix::NoPrefix;
use tw_keypair::ecdsa::secp256k1;
use tw_proto::Ethereum::Proto;
use tw_proto::TxCompiler::Proto as CompilerProto;

pub struct EthereumEntry;

impl CoinEntry for EthereumEntry {
    type AddressPrefix = NoPrefix;
    type Address = Address;
    type SigningInput<'a> = Proto::SigningInput<'a>;
    type SigningOutput = Proto::SigningOutput<'static>;
    type PreSigningOutput = CompilerProto::PreSigningOutput<'static>;

    // Optional modules:
    type JsonSigner = EthJsonSigner;
    type InputBuilder = EthInputBuilder;
    type PlanBuilder = NoPlanBuilder;

    fn parse_address(
        &self,
        _coin: &dyn CoinContext,
        address: &str,
        _prefix: Option<Self::AddressPrefix>,
    ) -> AddressResult<Self::Address> {
        Address::from_str(address)
    }

    fn derive_address(
        &self,
        _coin: &dyn CoinContext,
        public_key: PublicKeyBytes,
        _derivation: Derivation,
        _prefix: Option<Self::AddressPrefix>,
    ) -> AddressResult<Self::Address> {
        let public_key = secp256k1::PublicKey::try_from(public_key.as_slice())
            .map_err(|_| AddressError::PublicKeyTypeMismatch)?;
        Ok(Address::with_secp256k1_pubkey(&public_key))
    }

    fn sign(&self, _coin: &dyn CoinContext, input: Self::SigningInput<'_>) -> Self::SigningOutput {
        Signer::sign_proto(input)
    }

    fn preimage_hashes(
        &self,
        _coin: &dyn CoinContext,
        input: Self::SigningInput<'_>,
    ) -> Self::PreSigningOutput {
        Compiler::preimage_hashes(input)
    }

    fn compile(
        &self,
        _coin: &dyn CoinContext,
        input: Self::SigningInput<'_>,
        signatures: Vec<SignatureBytes>,
        public_keys: Vec<PublicKeyBytes>,
    ) -> Self::SigningOutput {
        Compiler::compile(input, signatures, public_keys)
    }

    fn json_signer(&self) -> Option<Self::JsonSigner> {
        Some(EthJsonSigner)
    }

    fn signing_input_builder(&self) -> Option<Self::InputBuilder> {
        Some(EthInputBuilder)
    }
}
