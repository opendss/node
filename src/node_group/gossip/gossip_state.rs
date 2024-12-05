use crate::common::selector::selector::{RandomIndexSelector, RoundRobinIndexSelector};
use crate::node_group::NodeStats;
use dashmap::DashMap;
use std::sync::{atomic, Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct State {
    inner: Arc<Inner>,
}
struct Inner {
    sequence_generator: atomic::AtomicU64,
    nodes_index: Mutex<NodesIndex>,
}
struct NodesIndex {
    nodes: DashMap<String, Arc<NodeStats>>,
    node_name_index: Vec<String>,
    round_robin_node_index_selector: RoundRobinIndexSelector,
    random_node_index_selector: RandomIndexSelector,
}

impl State {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                sequence_generator: atomic::AtomicU64::new(0),
                nodes_index: Mutex::new(NodesIndex {
                    nodes: DashMap::new(),
                    node_name_index: Vec::new(),
                    round_robin_node_index_selector: RoundRobinIndexSelector::new(0),
                    random_node_index_selector: RandomIndexSelector::new(0),
                }),
            }),
        }
    }

    pub fn next_seq(&mut self) -> u64 {
        self.inner
            .sequence_generator
            .fetch_add(1, atomic::Ordering::SeqCst)
    }

    pub async fn round_robin_next(&self) -> Option<Arc<NodeStats>> {
        let mut index_guard = self.inner.nodes_index.lock().await;
        let index = index_guard.round_robin_node_index_selector.next();
        let option = index_guard.node_name_index.get(index as usize);
        if let Some(name) = option {
            let node = index_guard.nodes.get(name);
            if let Some(entry) = node {
                return Some(entry.value().clone());
            }
        }
        None
    }

    pub async fn random_nodes(&self, num: u32) -> Vec<Arc<NodeStats>> {
        let mut vec = Vec::new();
        let mut index_guard = self.inner.nodes_index.lock().await;
        // todo: avoid busy loop
        let indexes = index_guard.random_node_index_selector.next(num);
        for index in indexes {
            let option = index_guard.node_name_index.get(index as usize);
            if let Some(name) = option {
                if let Some(entry) = index_guard.nodes.get(name) {
                    vec.push(entry.value().clone())
                }
            } 
        }
        vec
    }
}
