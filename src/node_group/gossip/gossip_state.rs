use crate::node_group::NodeStats;
use dashmap::DashMap;
use std::sync::atomic;

pub struct State {
    pub(crate) sequence_generator: atomic::AtomicU64,
    nodes: DashMap<String, NodeStats>,
}
impl State {
    pub fn new() -> Self {
        Self {
            sequence_generator: atomic::AtomicU64::new(0),
            nodes: DashMap::new(),
        }
    }
}
