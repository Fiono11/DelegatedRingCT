// Copyright (c) 2018-2022 The MobileCoin Foundation

use super::{Error, OneTimeKeyDeriveData, RingSigner, SignableInputRing};
use mc_crypto_keys::RistrettoPublic;
use mc_crypto_ring_signature::{TriptychSignature, Sign};
use std::string::String;
use curve25519_dalek::ristretto::RistrettoPoint;
use log::debug;

/// An implementation of RingSigner that holds no keys, and doesn't do any
/// non-trivial derivation of the one-time private key.
///
/// This version only works if the input secret actually includes the one-time
/// private key, and returns an error if only the alternative is supplied.
///
/// The purpose of this is to avoid the need to refactor existing working
/// software like SDKs that would not benefit from migrating to the
/// LocalRingSigner, at least for now
#[derive(Clone, Debug)]
pub struct NoKeysRingSigner {}

impl RingSigner for NoKeysRingSigner {
    fn sign(
        &self,
        message: &[u8],
        ring: &SignableInputRing,
        //pseudo_output_blinding: Scalar,
        //rng: &mut dyn CryptoRngCore,
    ) -> Result<TriptychSignature, Error> {
        let real_input = ring
            .members
            .get(ring.real_input_index)
            .ok_or(Error::RealInputIndexOutOfBounds)?;
        let target_key = RistrettoPublic::try_from(&real_input.target_key)?;

        // First, get the one-time private key
        let onetime_private_key = match ring.input_secret.onetime_key_derive_data {
            OneTimeKeyDeriveData::OneTimeKey(key) => key,
            OneTimeKeyDeriveData::SubaddressIndex(_) => {
                return Err(Error::NoPathToSpendKey);
            }
        };

        //debug!("NO KEYS SIGNER");
        // Check if this is the correct one-time private key
        if RistrettoPublic::from(&onetime_private_key) != target_key {
            return Err(Error::TrueInputNotOwned);
        }

        let ring: Vec<RistrettoPoint> = ring.members.iter().map(|x| x.target_key.0.decompress().unwrap()).collect();

        let signature = Sign(&onetime_private_key.0, &String::from_utf8_lossy(message), &ring);

        // Sign the TriptychSignature
        Ok(signature)
    }
}