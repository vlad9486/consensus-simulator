fn main() {
    use std::time::Duration;
    use consensus_simulator::{Simulator, Network, TenderbakeNode, TenderbakeConfig};

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
            let r = rand::Rng::gen::<u8>(&mut rand::thread_rng()) / 6;
            Duration::from_millis(100 + (r as u64))
        }
    }

    let seed = 0x123456;
    let configs = TenderbakeConfig::new(16, Duration::from_secs(3), seed);
    let simulator = Simulator::new(configs.map(TenderbakeNode::new), DefaultNetwork);
    simulator.run(1000);
}
