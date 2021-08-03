use std::time::Duration;
use super::{blockchain, config::TenderbakeConfig};
use crate::{Node, NodeEvent, Effect};

#[derive(Debug, Clone)]
enum Payload {
    Propose(blockchain::Chain),
    Preendorse(blockchain::Preendorsement),
    Endorse(blockchain::Endorsement, Vec<blockchain::Preendorsement>),
    Preendorsements(blockchain::Block, Vec<blockchain::Preendorsement>),
}

#[derive(Debug, Clone)]
pub struct Msg {
    level: u64,
    round_id: u64,
    previous_block_hash: Option<blockchain::BlockHash>,
    payload: Payload,
}

enum ProposalState {
    NoProposal,
    CollectingPreendorsements {
        acc: Vec<blockchain::Preendorsement>,
    },
    CollectingEndorsements {
        pqc: Vec<blockchain::Preendorsement>,
        acc: Vec<blockchain::Endorsement>,
    },
}

pub struct TenderbakeNode {
    config: TenderbakeConfig,
    chain: blockchain::Chain,
    proposal_state: ProposalState,
    endorsable: Option<blockchain::Round>,
    locked: Option<blockchain::Round>,
}

impl TenderbakeNode {
    pub fn new(config: TenderbakeConfig) -> Self {
        TenderbakeNode {
            config,
            chain: blockchain::Chain::default(),
            proposal_state: ProposalState::NoProposal,
            endorsable: None,
            locked: None,
        }
    }
}

fn is_proposer(node_id: usize, total_nodes: usize, level: u64, round_id: u64) -> bool {
    node_id == ((level + round_id) as usize) % total_nodes
}

impl Node for TenderbakeNode {
    type Message = blockchain::Signed<Msg>;

    fn handle_event(&mut self, time: Duration, event: &NodeEvent<Self::Message>) -> Vec<Effect<Self::Message>> {
        match event {
            &NodeEvent::MessageReceived(ref message) => {
                if let Some((msg, signer_id)) = message.clone().check() {
                    match msg.payload {
                        Payload::Propose(candidate_chain) => {
                            let is_proposer_valid = is_proposer(
                                signer_id,
                                self.config.total_nodes,
                                msg.level,
                                msg.round_id,
                            );
                            let previously_proposed_pqc_is_correct = match candidate_chain.head() {
                                None => true,
                                Some(head) => head.is_pqc_valid(&self.config),
                            };
                            let round = self.endorsable.as_ref();
                            let ok = true
                                && is_proposer_valid
                                && previously_proposed_pqc_is_correct
                                && candidate_chain.valid_chain(&self.config)
                                && candidate_chain.better_chain(&self.chain, round);
                            if ok {
                                let candidate_timestamp = candidate_chain
                                    .head()
                                    .expect("impossible")
                                    .timestamp();
                                let _ = candidate_timestamp;
                                unimplemented!()
                            } else {
                                vec![]
                            }
                        },
                        Payload::Preendorse(blockchain::Preendorsement(p)) => {
                            let _ = p;
                            unimplemented!()
                        },
                        Payload::Endorse(blockchain::Endorsement(e), pqc) => {
                            let _ = (e, pqc);
                            unimplemented!()
                        },
                        Payload::Preendorsements(block, pqc) => {
                            let _ = (block, pqc);
                            unimplemented!()
                        }
                    }
                } else {
                    vec![]
                }
            },
            NodeEvent::WakeUp => {
                let _ = time;
                vec![]
            },
        }
    }
}
