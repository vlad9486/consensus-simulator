fn main() {
    use std::time::Duration;
    use consensus_simulator::{Simulator, Network, Node, NodeEvent, Effect};

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

    struct DummyNode;

    #[derive(Debug, Clone)]
    struct DummyMessage;

    impl Node for DummyNode {
        type Message = DummyMessage;

        fn handle_event(
            &mut self,
            time: Duration,
            event: &NodeEvent<Self::Message>,
        ) -> Vec<Effect<Self::Message>> {
            let _ = (self, time, event);
            Vec::new()
        }
    }

    let simulator = Simulator::new((0..16).map(|_| DummyNode), DefaultNetwork);
    simulator.run(1000);
}
