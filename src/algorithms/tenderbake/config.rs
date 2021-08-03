use std::time::Duration;

pub struct TenderbakeConfig {
    pub id: usize,
    pub total_nodes: usize,
    pub quorum_size: usize,
    pub round0_duration: Duration,
    pub seed: u64,
}

impl TenderbakeConfig {
    pub fn new(
        total_nodes: usize,
        round0_duration: Duration,
        seed: u64,
    ) -> impl Iterator<Item = Self> {
        let quorum_size = total_nodes / 3 + 1;
        let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(seed);
        (0..total_nodes)
            .map(move |id| TenderbakeConfig {
                id,
                total_nodes,
                quorum_size,
                round0_duration,
                seed: rand::Rng::gen(&mut rng),
            })
    }
}
