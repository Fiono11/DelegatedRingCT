// Copyright(C) Facebook, Inc. and its affiliates.
use crate::error::{DagError, DagResult};
use crate::messages::{Certificate, Header, Vote, Hash};
use config::{Committee, Stake, PK};
use mc_account_keys::PublicAddress;
use mc_crypto_keys::RistrettoSignature;
use mc_crypto_keys::tx_hash::TxHash;
use std::collections::HashSet;

/// Aggregates votes for a particular header into a certificate.
pub struct VotesAggregator {
    weight: Stake,
    votes: Vec<(PublicAddress, RistrettoSignature)>,
    used: HashSet<PublicAddress>,
}

impl VotesAggregator {
    pub fn new() -> Self {
        Self {
            weight: 0,
            votes: Vec::new(),
            used: HashSet::new(),
        }
    }

    pub fn append(
        &mut self,
        vote: Vote,
        committee: &Committee,
        header: &Header,
    ) -> DagResult<Option<Certificate>> {
        let author = vote.author;

        // Ensure it is the first time this authority votes.
        ensure!(self.used.insert(author.clone()), DagError::AuthorityReuse(author.clone()));

        self.votes.push((author.clone(), vote.signature));
        self.weight += committee.stake(&PK(author.to_bytes()));
        if self.weight >= committee.quorum_threshold() {
            self.weight = 0; // Ensures quorum is only reached once.
            return Ok(Some(Certificate {
                header: header.clone(),
                votes: self.votes.clone(),
            }));
        }
        Ok(None)
    }
}

/// Aggregate certificates and check if we reach a quorum.
pub struct CertificatesAggregator {
    weight: Stake,
    certificates: Vec<TxHash>,
    used: HashSet<PublicAddress>,
}

impl CertificatesAggregator {
    pub fn new() -> Self {
        Self {
            weight: 0,
            certificates: Vec::new(),
            used: HashSet::new(),
        }
    }

    pub fn append(
        &mut self,
        certificate: Certificate,
        committee: &Committee,
    ) -> DagResult<Option<Vec<TxHash>>> {
        let origin = certificate.origin();

        // Ensure it is the first time this authority votes.
        if !self.used.insert(origin.clone()) {
            return Ok(None);
        }

        self.certificates.push(certificate.digest());
        self.weight += committee.stake(&PK(origin.to_bytes()));
        if self.weight >= committee.quorum_threshold() {
            self.weight = 0; // Ensures quorum is only reached once.
            return Ok(Some(self.certificates.drain(..).collect()));
        }
        Ok(None)
    }
}
