use std::time::Duration;
use crypto::blake2b;
use tezos_messages::p2p::binary_message::BinaryWrite;
use tezos_encoding::encoding::HasEncoding;
use serde::Serialize;
use super::config::TenderbakeConfig;

#[derive(Debug, Clone, HasEncoding, Serialize, PartialEq, Eq)]
pub struct BlockHash {
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, HasEncoding, Serialize)]
pub struct Transaction {
    #[encoding(builtin = "Uint32")]
    x: u32,
}

#[derive(Debug, Clone, HasEncoding, Serialize)]
pub struct BlockContents {
    transactions: Vec<Transaction>,
    #[encoding(builtin = "Uint32")]
    level: u32,
    predecessor_hash: Option<BlockHash>,
}

impl BlockContents {
    pub fn hash(&self) -> BlockHash {
        BlockHash {
            bytes: blake2b::digest_256(&self.as_bytes().unwrap()).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Signature(usize);

#[derive(Debug, Clone)]
pub struct Signed<T>(Signature, T);

impl<T> Signed<T> {
    pub fn check(self) -> Option<(T, usize)> {
        let Signed(Signature(signer_id), t) = self;
        Some((t, signer_id))
    }
}

#[derive(Debug, Clone)]
pub struct Preendorsement(pub Signed<BlockHash>);

impl AsRef<Signed<BlockHash>> for Preendorsement {
    fn as_ref(&self) -> &Signed<BlockHash> {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Endorsement(pub Signed<BlockHash>);

impl AsRef<Signed<BlockHash>> for Endorsement {
    fn as_ref(&self) -> &Signed<BlockHash> {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    contents: BlockContents,
    round: u64,
    timestamp: Duration,
    predecessor_eqc: Vec<Endorsement>,
    previously_proposed: Option<(u64, Vec<Preendorsement>)>
}

fn is_committee_member(node_id: usize, total_nodes: usize, level: u32) -> bool {
    let _ = (node_id, total_nodes, level);
    true
}

impl Block {
    pub fn is_pqc_valid(&self, config: &TenderbakeConfig) -> bool {
        if let &Some((_, ref pqc)) = &self.previously_proposed {
            self.is_qc_valid(config, pqc.into_iter())
        } else {
            true
        }
    }

    pub fn is_eqc_valid(&self, config: &TenderbakeConfig, eqc: &[Endorsement]) -> bool {
        self.is_qc_valid(config, eqc.into_iter())
    }

    fn is_qc_valid<'a, C>(
        &self,
        config: &TenderbakeConfig,
        qc: impl Iterator<Item = &'a C> + 'a,
    ) -> bool
    where
        C: 'a + AsRef<Signed<BlockHash>>,
    {
        let this_hash = self.contents.hash();
        for signed in qc {
            if let Some((hash, signer_id)) = signed.as_ref().clone().check() {
                let ok = true
                    && is_committee_member(signer_id, config.total_nodes, self.contents.level)
                    && this_hash == hash;
                if !ok {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }
}

#[derive(Debug, Clone, Default)]
pub struct Chain(Vec<Block>);

impl Chain {
    pub fn head(&self) -> Option<&Block> {
        self.0.last()
    }

    pub fn valid_chain(&self, config: &TenderbakeConfig) -> bool {
        fn inner(
            chain: &[Block],
            config: &TenderbakeConfig,
            eqc: Option<&[Endorsement]>,
            hash: Option<Option<BlockHash>>,
        ) -> bool {
            if let Some((block, rest)) = chain.split_last() {
                let is_eqc_correct = match eqc {
                    None => true,
                    Some(eqc) => block.is_eqc_valid(config, eqc),
                };
                let is_hash_correct = match hash {
                    None => true,
                    Some(None) => block.contents.level == 1,
                    Some(Some(hash)) => hash == block.contents.hash(),
                };
                let rest = inner(
                    rest,
                    config,
                    Some(block.predecessor_eqc.as_slice()),
                    Some(block.contents.predecessor_hash.clone()),
                );
                is_hash_correct && is_eqc_correct && rest
            } else {
                true
            }
        }

        inner(&self.0, config, None, None)
    }

    fn level(&self) -> u32 {
        match self.0.last() {
            None => 0,
            Some(head) => head.contents.level,
        }
    }

    pub fn better_chain(&self, current: &Self, round: Option<&Round>) -> bool {
        if self.level() == current.level() {
            match (self.0.split_last(), current.0.split_last()) {
                (Some((candidate_head, candidate_rest)), Some((_, node_rest))) => {
                    let node_predecessor_round_is_higher =
                    match (candidate_rest.split_last(), node_rest.split_last()) {
                        (Some((candidate_predecessor, _)), Some((node_predecessor, _))) => {
                            node_predecessor.round >= candidate_predecessor.round
                        },
                        (None, None) => true,
                        _ => false,
                    };

                    match (candidate_head.previously_proposed, round) {
                        (None, None) => node_predecessor_round_is_higher,
                        (Some((candidate_endorsable_round, _)), Some((node_endorsable_round, _))) => {
                            if candidate_endorsable_round == node_endorsable_round {
                                node_predecessor_round_is_higher
                            } else {
                                candidate_endorsable_round > node_endorsable_round
                            }
                        },
                        (Some(_), None) => true,
                        (None, Some(_)) => false,
                    }

                    let _ = (current, round);
                    if let Some(round) = round {
                        let _ = (&round.round_id, &round.block_contents, &round.quorum_certificate);
                    }
                    unimplemented!()
                },
                _ => false,
            }
        } else {
            self.level() > current.level()
        }
    }
}

pub struct Round {
    round_id: usize,
    block_contents: BlockContents,
    quorum_certificate: Vec<Preendorsement>,
}
