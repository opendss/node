use crate::node_group::NodeStats;
use dashmap::DashMap;
use std::sync::{atomic, Arc};

#[derive(Clone)]
pub struct State {
    inner: Arc<Inner>,
}

struct Inner {
    sequence_generator: atomic::AtomicU64,
    nodes: DashMap<String, NodeStats>,
    node_name_index: Vec<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                sequence_generator: atomic::AtomicU64::new(0),
                nodes: DashMap::new(),
                node_name_index: Vec::new(),
            }),
        }
    }
}
