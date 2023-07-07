// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::transaction::signature::EthSignature;
use tw_hash::{sha3::keccak256, H256};
use tw_keypair::ecdsa::secp256k1;
use tw_number::U256;

pub mod signature;
pub mod transaction_eip1559;
pub mod transaction_non_typed;
pub mod user_operation;

pub trait TransactionCommon {
    fn payload(&self) -> Vec<u8>;
}

pub trait UnsignedTransaction: TransactionCommon {
    type SignedTransaction: SignedTransaction + 'static;

    fn pre_hash(&self, chain_id: U256) -> H256 {
        let hash = keccak256(&self.encode(chain_id));
        H256::try_from(hash.as_slice()).expect("keccak256 returns 32 bytes")
    }

    fn encode(&self, chain_id: U256) -> Vec<u8>;

    fn into_signed(
        self,
        signature: secp256k1::Signature,
        chain_id: U256,
    ) -> Self::SignedTransaction;
}

pub trait SignedTransaction: TransactionCommon {
    type Signature: EthSignature;

    fn hash(&self) -> H256 {
        let hash = keccak256(&self.encode());
        H256::try_from(hash.as_slice()).expect("keccak256 returns 32 bytes")
    }

    fn encode(&self) -> Vec<u8>;

    fn signature(&self) -> &Self::Signature;
}

pub trait UnsignedTransactionBox: TransactionCommon {
    fn into_boxed(self) -> Box<dyn UnsignedTransactionBox + 'static>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    fn pre_hash(&self, chain_id: U256) -> H256;

    fn encode(&self, chain_id: U256) -> Vec<u8>;

    fn into_signed(
        self: Box<Self>,
        signature: secp256k1::Signature,
        chain_id: U256,
    ) -> Box<dyn SignedTransactionBox>;
}

impl<T> UnsignedTransactionBox for T
where
    T: UnsignedTransaction,
{
    fn pre_hash(&self, chain_id: U256) -> H256 {
        <Self as UnsignedTransaction>::pre_hash(self, chain_id)
    }

    fn encode(&self, chain_id: U256) -> Vec<u8> {
        <Self as UnsignedTransaction>::encode(self, chain_id)
    }

    fn into_signed(
        self: Box<Self>,
        signature: secp256k1::Signature,
        chain_id: U256,
    ) -> Box<dyn SignedTransactionBox> {
        Box::new(<Self as UnsignedTransaction>::into_signed(
            *self, signature, chain_id,
        ))
    }
}

pub trait SignedTransactionBox: TransactionCommon {
    fn hash(&self) -> H256;

    fn encode(&self) -> Vec<u8>;

    fn signature(&self) -> &dyn EthSignature;
}

impl<T> SignedTransactionBox for T
where
    T: SignedTransaction,
{
    fn hash(&self) -> H256 {
        <Self as SignedTransaction>::hash(self)
    }

    fn encode(&self) -> Vec<u8> {
        <Self as SignedTransaction>::encode(self)
    }

    fn signature(&self) -> &dyn EthSignature {
        <Self as SignedTransaction>::signature(self)
    }
}
