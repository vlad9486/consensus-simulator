
use std::{collections::BinaryHeap, fmt};
use super::{
    node::{Node, NodeEvent, Effect},
    event::EventItem,
    node_state::NodeState,
    network::Network,
};

pub struct Simulator<N, D>
where
    N: Node,
{
    node_pool: Vec<NodeState<N>>,
    event_heap: BinaryHeap<EventItem<NodeEvent<N::Message>>>,
    network: D,
}

impl<N, D> Simulator<N, D>
where
    N: Node,
    N::Message: fmt::Debug,
    D: Network,
{
    pub fn new(nodes: impl Iterator<Item = N>, network: D) -> Self {
        Simulator {
            node_pool: nodes.map(NodeState::new).collect(),
            event_heap: BinaryHeap::new(),
            network,
        }
    }

    fn pop_valid_event(&mut self) -> Option<EventItem<NodeEvent<N::Message>>> {
        let next_event_time = self.event_heap.peek().map(|e| e.time());
        let min_wake_up = self.node_pool
            .iter_mut()
            .filter(|s| s.wake_up_time().is_some())
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(&b));

        match (next_event_time, min_wake_up) {
            (None, None) => None,
            (Some(_), None) => self.event_heap.pop(),
            (None, Some((node_id, state))) => {
                let time = state.clear_wake_up_time().unwrap();
                Some(EventItem::new(time, 0, 0, node_id, NodeEvent::WakeUp))
            },
            (Some(event_time), Some((node_id, state))) => {
                if event_time < state.wake_up_time().unwrap() {
                    self.event_heap.pop()
                } else {
                    let time = state.clear_wake_up_time().unwrap();
                    Some(EventItem::new(time, 0, 0, node_id, NodeEvent::WakeUp))
                }
            },
        }
    }

    fn handle_event(&mut self, event: EventItem<NodeEvent<N::Message>>, cnt: usize) {
        let node_number = self.node_pool.len();
        let node_id = event.node_id();
        let this_time = event.time();
        let state = self.node_pool
            .get_mut(node_id)
            .expect(&format!("event {:?} for node that doesn't exist", event));
        let effects = state.handle_event(event);

        for (effect_index, effect) in effects.into_iter().enumerate() {
            match effect {
                Effect::ShutDown => state.shut_down(),
                Effect::SetWakeUpTime(time) => state.set_wake_up_time(time),
                Effect::Broadcast(message) => {
                    for i in 0..node_number {
                        let event = NodeEvent::MessageReceived(message.clone());
                        if i != node_id {
                            let e = effect_index;
                            let new_time = self.network.delay(this_time, cnt, e, node_id, i);
                            let item = EventItem::new(new_time, cnt, e, i, event);
                            self.event_heap.push(item);
                        }
                    }
                }
            }
        }
    }

    pub fn run(mut self, iterations_number: usize) {
        let mut cnt = 0;
        while let Some(event) = self.pop_valid_event() {
            self.handle_event(event, cnt);
            cnt += 1;
            if cnt == iterations_number {
                break;
            }
        }
    }
}
