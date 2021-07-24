use std::time::Duration;
use crate::{Node, NodeEvent, Effect};

pub struct TenderbakeNode {

}

impl TenderbakeNode {
    pub fn new() -> Self {
        TenderbakeNode {}
    }
}

impl Node for TenderbakeNode {
    type Message = ();

    fn handle_event(&mut self, time: Duration, event: &NodeEvent<Self::Message>) -> Vec<Effect<Self::Message>> {
        let _ = (time, event);
        vec![]
    }
}
