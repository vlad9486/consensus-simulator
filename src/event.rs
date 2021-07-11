use std::{cmp::Ordering, time::Duration};

#[derive(Debug)]
pub struct EventItem<E> {
    metadata: EventMeta,
    node_id: usize,
    inner: E,
}

impl<E> EventItem<E> {
    pub fn new(
        time: Duration,
        iteration: usize,
        effect_index: usize,
        node_id: usize,
        event: E,
    ) -> Self {
        EventItem {
            metadata: EventMeta {
                time,
                iteration,
                effect_index,
            },
            node_id,
            inner: event,
        }
    }

    pub fn node_id(&self) -> usize {
        self.node_id
    }

    pub fn time(&self) -> Duration {
        self.metadata.time
    }

    pub fn event(&self) -> &E {
        &self.inner
    }
}

impl<E> PartialEq for EventItem<E> {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.eq(&other.metadata)
    }
}

impl<E> Eq for EventItem<E> {}

impl<E> PartialOrd for EventItem<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.metadata.partial_cmp(&other.metadata).map(Ordering::reverse)
    }
}

impl<E> Ord for EventItem<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.metadata.cmp(&other.metadata).reverse()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EventMeta {
    time: Duration,
    iteration: usize,
    effect_index: usize,
}
