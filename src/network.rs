use std::time::Duration;

pub trait Network {
    fn delay(
        &self,
        this: Duration,
        iteration: usize,
        effect_index: usize,
        sender_node_id: usize,
        receiver_node_id: usize,
    ) -> Duration;
}
