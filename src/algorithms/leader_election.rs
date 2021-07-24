use std::time::Duration;
use crate::{Node, NodeEvent, Effect};

pub struct LeaderElectionNode {
    num: usize,
    inner: State,
}

enum State {
    Good(GoodNode),
}

enum GoodNode {
    Alive {
        this_id: usize,
        seen: Vec<usize>,
    },
    Decided {
        leader: usize,
    },
}

impl LeaderElectionNode {
    pub fn good(this_id: usize, num: usize) -> Self {
        LeaderElectionNode {
            inner: State::Good(GoodNode::Alive {
                this_id,
                seen: vec![this_id],
            }),
            num,
        }
    }
}

impl Node for LeaderElectionNode {
    type Message = usize;

    fn handle_event(
        &mut self,
        time: Duration,
        event: &NodeEvent<Self::Message>,
    ) -> Vec<Effect<Self::Message>> {
        let _ = time;
        let num = self.num;
        match &mut self.inner {
            &mut State::Good(ref mut state) => match state {
                &mut GoodNode::Alive { ref this_id, ref mut seen } => {
                    match event {
                        &NodeEvent::WakeUp => vec![Effect::Broadcast(*this_id)],
                        &NodeEvent::MessageReceived(id) => {
                            seen.push(id);
                            if seen.len() == num {
                                *state = GoodNode::Decided {
                                    leader: *seen.iter().max().unwrap(),
                                };
                                vec![]
                            } else {
                                vec![]
                            }
                        }
                    }
                },
                &mut GoodNode::Decided { leader } => {
                    let _ = leader;
                    vec![]
                }
            }
        }
    }
}
