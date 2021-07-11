use std::{time::Duration, fmt, cmp::Ordering};
use super::{
    node::{Node, NodeEvent, Effect},
    event::EventItem,
};

pub struct NodeState<N> {
    node: Option<N>,
    wake_up_time: Option<Duration>,
}

impl<N> NodeState<N> {
    pub fn new(node: N) -> Self {
        NodeState {
            node: Some(node),
            wake_up_time: None,
        }
    }

    pub fn shut_down(&mut self) {
        self.node = None;
    }

    pub fn set_wake_up_time(&mut self, time: Duration) {
        self.wake_up_time = Some(time);
    }

    pub fn clear_wake_up_time(&mut self) -> Option<Duration> {
        self.wake_up_time.take()
    }

    pub fn wake_up_time(&self) -> Option<Duration> {
        self.wake_up_time
    }
}

impl<N> NodeState<N>
where
    N: Node,
    N::Message: fmt::Debug,
{
    pub fn handle_event(&mut self, event: EventItem<NodeEvent<N::Message>>) -> Vec<Effect<N::Message>> {
        self.node
            .as_mut()
            .expect(&format!("node was shutdown, cannot handle {:?}", event))
            .handle_event(event.time(), event.event())
    }
}

impl<N> PartialEq for NodeState<N> {
    fn eq(&self, other: &Self) -> bool {
        self.wake_up_time.eq(&other.wake_up_time)
    }
}

impl<N> Eq for NodeState<N> {
}

impl<N> PartialOrd for NodeState<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.wake_up_time.partial_cmp(&other.wake_up_time)
    }
}

impl<N> Ord for NodeState<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wake_up_time.cmp(&other.wake_up_time)
    }
}
