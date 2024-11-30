use crate::node_group::gossip::broadcast::broadcast_manager_policy::BroadCastPolicy;
use crate::node_group::RawPacket;
use prost::Message;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::{atomic, Arc, RwLock};

struct BroadCastMessage {
    key: String,
    packet: RawPacket,
    transmit_count: u32,
    invalid: atomic::AtomicBool,
}
impl BroadCastMessage {
    fn invalid(&self) {
        self.invalid.store(true, atomic::Ordering::Release);
    }
}
impl Eq for BroadCastMessage {}

impl PartialEq<Self> for BroadCastMessage {
    fn eq(&self, other: &Self) -> bool {
        self.packet.eq(&other.packet)
            && self.transmit_count == other.transmit_count
            && self.invalid.load(atomic::Ordering::Acquire)
                == other.invalid.load(atomic::Ordering::Acquire)
    }
}

impl PartialOrd<Self> for BroadCastMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BroadCastMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.transmit_count.cmp(&other.transmit_count)
    }
}

const INIT_TRANSMIT: u32 = 0;
pub(crate) struct BroadCastManager {
    memory_cache: Arc<RwLock<MemoryCache>>,
    policy: BroadCastPolicy,
}

pub(crate) struct MemoryCache {
    queue: BinaryHeap<Arc<BroadCastMessage>>,
    index: HashMap<String, Arc<BroadCastMessage>>,
}

impl Default for MemoryCache {
    fn default() -> Self {
        MemoryCache {
            queue: BinaryHeap::new(),
            index: HashMap::new(),
        }
    }
}

impl BroadCastManager {
    fn len(&self) -> usize {
        self.memory_cache.read().unwrap().index.len()
    }
    fn offer(&mut self, key: String, message: RawPacket) {
        let message_arc = Arc::new(BroadCastMessage {
            key: key.clone(),
            packet: message,
            transmit_count: INIT_TRANSMIT,
            invalid: atomic::AtomicBool::new(false),
        });
        let mut memory_cache = self.memory_cache.write().unwrap();
        let previous = memory_cache.index.insert(key, message_arc.clone());
        if let Some(previous) = previous {
            previous.invalid()
        }
        memory_cache.queue.push(message_arc);
    }

    fn get_broadcast_messages(&mut self, limit_size: usize) -> Vec<Arc<BroadCastMessage>> {
        let mut vec = Vec::new();
        let mut memory_cache = self.memory_cache.write().unwrap();
        let mut current_size = 0;
        while current_size < limit_size {
            let option = memory_cache.queue.peek();
            if option.is_none() {
                break;
            }
            let retransmit_limit = self.policy.get_retransmit_limit();
            let message = option.unwrap();
            if message.invalid.load(atomic::Ordering::Acquire) {
                memory_cache.queue.pop();
                continue;
            }

            if message.transmit_count + 1 >= retransmit_limit {
                continue;
            }

            let encode_len = message.packet.encoded_len();
            current_size += encode_len;
            if current_size > limit_size {
                break;
            }
            let message = memory_cache.queue.pop().unwrap();
            memory_cache.index.remove(&message.key);
            vec.push(message)
        }
        vec
    }

    pub fn new(policy: BroadCastPolicy) -> Self {
        Self {
            memory_cache: Arc::new(RwLock::new(MemoryCache {
                queue: BinaryHeap::new(),
                index: HashMap::new(),
            })),
            policy,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node_group::gossip::broadcast::broadcast_manager::BroadCastManager;
    use crate::node_group::gossip::broadcast::broadcast_manager_policy::BroadCastPolicy;
    use crate::node_group::raw_packet::Inner;
    use crate::node_group::{PingPacket, RawPacket};

    #[test]
    fn test_broadcast_manager_remove_same_key() {
        let policy = BroadCastPolicy {
            node_num: 3,
            retransmit_quota: 3,
        };
        // init manager
        let mut manager = BroadCastManager::new(policy);

        // offer some messages
        manager.offer(
            String::from("node_1"),
            RawPacket {
                inner: Some(Inner::PingPacket(PingPacket {
                    seq: 0,
                    source_node_id: String::from("node-1"),
                    target_node_id: String::from("node-2"),
                    source_address: String::from("node-3"),
                    target_address: None,
                })),
            },
        );

        manager.offer(
            String::from("node_1"),
            RawPacket {
                inner: Some(Inner::PingPacket(PingPacket {
                    seq: 1,
                    source_node_id: String::from("node-1"),
                    target_node_id: String::from("node-2"),
                    source_address: String::from("node-3"),
                    target_address: None,
                })),
            },
        );

        let vec = manager.get_broadcast_messages(1024);
        assert_eq!(vec.len(), 1);
        match &vec[0].packet.inner.as_ref().unwrap() {
            Inner::PingPacket(ping) => {
                assert_eq!(ping.seq, 1)
            }
            _ => {
                panic!("unexpected packet type");
            }
        }
    }

    #[test]
    fn test_broadcast_manager_get_messages_by_size() {
        let policy = BroadCastPolicy {
            node_num: 3,
            retransmit_quota: 3,
        };
        // init manager
        let mut manager = BroadCastManager::new(policy);

        // offer some messages
        manager.offer(
            String::from("node_2"),
            RawPacket {
                inner: Some(Inner::PingPacket(PingPacket {
                    seq: 0,
                    source_node_id: String::from("node-1"),
                    target_node_id: String::from("node-2"),
                    source_address: String::from("node-3"),
                    target_address: None,
                })),
            },
        );

        manager.offer(
            String::from("node_3"),
            RawPacket {
                inner: Some(Inner::PingPacket(PingPacket {
                    seq: 1,
                    source_node_id: String::from("node-1"),
                    target_node_id: String::from("node-2"),
                    source_address: String::from("node-3"),
                    target_address: None,
                })),
            },
        );

        assert_eq!(manager.len(), 2);
        let vec = manager.get_broadcast_messages(30);
        assert_eq!(vec.len(), 1);
        assert_eq!(manager.len(), 1);
        let vec = manager.get_broadcast_messages(30);
        assert_eq!(vec.len(), 1);
        assert_eq!(manager.len(), 0);
    }
}
