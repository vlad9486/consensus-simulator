use std::time::Duration;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeEvent<M> {
    WakeUp,
    MessageReceived(M),
}

pub enum Effect<M> {
    ShutDown,
    SetWakeUpTime(Duration),
    Broadcast(M),
}

pub trait Node {
    type Message: Clone;

    fn handle_event(
        &mut self,
        time: Duration,
        event: &NodeEvent<Self::Message>,
    ) -> Vec<Effect<Self::Message>>;
}
