// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Domain separation tags for hash functions used in the MobileCoin amount and
//! MLSAG protocols.
//!
//! Domain separation allows multiple distinct hash functions to be derived from
//! a single base function:
//!   Hash_1(X) = Hash("Hash_1" || X),
//!   Hash_2(X) = Hash("Hash_2" || X),
//!   etc.
//!
//! Here, "Hash_1" and "Hash_2" are called domain separation tags. Tags should
//! uniquely identify the hash function within the protocol and may include the
//! protocol's version so that each derived hash function is independent of
//! others within the protocol and independent of hash functions in other
//! versions of the protocol.

/// Domain separator for onetime key "hash_to_point" function.
pub const HASH_TO_POINT_DOMAIN_TAG: &str = "mc_onetime_key_hash_to_point";

/// Domain separator for onetime key "hash_to_scalar" function.
pub const HASH_TO_SCALAR_DOMAIN_TAG: &str = "mc_onetime_key_hash_to_scalar";

/// Domain separator for RingMLSAG's challenges.
pub const RING_MLSAG_CHALLENGE_DOMAIN_TAG: &str = "mc_ring_mlsag_challenge";

/// Domain separator for Amount's value mask hash function.
pub const AMOUNT_VALUE_DOMAIN_TAG: &str = "mc_amount_value";

/// Domain separator for Amount's token_id mask hash function.
pub const AMOUNT_TOKEN_ID_DOMAIN_TAG: &str = "mc_amount_token_id";

/// Domain separator for Amount's blinding mask hash function.
pub const AMOUNT_BLINDING_DOMAIN_TAG: &str = "mc_amount_blinding";

/// Domain separator for Amount's shared-secret hash function.
pub const AMOUNT_SHARED_SECRET_DOMAIN_TAG: &str = "mc_amount_shared_secret";

/// Domain separator for Amount's blinding factors hkdf SALT
pub const AMOUNT_BLINDING_FACTORS_DOMAIN_TAG: &[u8] = b"mc_amount_blinding_factors";

/// Domain separator for Bulletproof transcript.
pub const BULLETPROOF_DOMAIN_TAG: &str = "mc_bulletproof_transcript";

/// Domain separator for hashing a TxOut leaf node in a Merkle tree.
pub const TXOUT_MERKLE_LEAF_DOMAIN_TAG: &str = "mc_tx_out_merkle_leaf";

/// Domain separator for hashing internal hash values in a Merkle tree.
pub const TXOUT_MERKLE_NODE_DOMAIN_TAG: &str = "mc_tx_out_merkle_node";

/// Domain separator for hashing the "nil" value in a Merkle tree.
pub const TXOUT_MERKLE_NIL_DOMAIN_TAG: &str = "mc_tx_out_merkle_nil";

/// Domain separator for computing the extended message digest
pub const EXTENDED_MESSAGE_DOMAIN_TAG: &str = "mc_extended_message";

/// Domain separator for computing the extended message and tx summary digest
pub const EXTENDED_MESSAGE_AND_TX_SUMMARY_DOMAIN_TAG: &str = "mc_extended_message_and_tx_summary";

/// Domain separator for hashing MintConfigTxPrefixs
pub const MINT_CONFIG_TX_PREFIX_DOMAIN_TAG: &str = "mc_mint_config_tx_prefix";

/// Domain separator for hashing MintTxPrefixs
pub const MINT_TX_PREFIX_DOMAIN_TAG: &str = "mc_mint_tx_prefix";

/// Domain separator for hashing a private view key and index into a subaddress.
pub(crate) const SUBADDRESS_DOMAIN_TAG: &str = "mc_subaddress";

/// An account's "default address" is its zero^th subaddress.
pub const DEFAULT_SUBADDRESS_INDEX: u64 = 0;

/// u64::MAX is a reserved subaddress value for "invalid/none" (MCIP #36)
pub const INVALID_SUBADDRESS_INDEX: u64 = u64::MAX;

/// An account's "change address" is the 1st reserved subaddress,
/// counting down from `u64::MAX`. (See MCIP #4, MCIP #36)
pub const CHANGE_SUBADDRESS_INDEX: u64 = u64::MAX - 1;

/// The subaddress derived using u64::MAX - 2 is the reserved subaddress
/// for gift code TxOuts to be sent as specified in MCIP #32.
pub const GIFT_CODE_SUBADDRESS_INDEX: u64 = u64::MAX - 2;