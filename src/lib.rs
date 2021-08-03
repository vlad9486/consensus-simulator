#![forbid(unsafe_code)]

mod node;
pub use self::node::{Node, NodeEvent, Effect};

mod event;

mod node_state;

mod network;
pub use self::network::Network;

mod simulator;
pub use self::simulator::Simulator;

mod algorithms;
pub use self::algorithms::leader_election::LeaderElectionNode;
#[cfg(feature = "tenderbake")]
pub use self::algorithms::tenderbake::{TenderbakeNode, TenderbakeConfig};
