fn main() {
    use std::time::Duration;
    use consensus_simulator::{Simulator, Network, TenderbakeNode};

    struct DefaultNetwork;

    impl Network for DefaultNetwork {
        fn delay(
            &self,
            this: Duration,
            iteration: usize,
            effect_index: usize,
            sender_node_id: usize,
            receiver_node_id: usize,
        ) -> Duration {
            let _ = (self, this, iteration, effect_index, sender_node_id, receiver_node_id);
            Duration::from_millis(100)
        }
    }

    let simulator = Simulator::new(
        (0..16).map(|_id| TenderbakeNode::new()),
        DefaultNetwork,
    );
    simulator.run(1000);
}
