#[derive(Clone)]
pub(crate) struct BroadCastPolicy {
    pub(crate) node_num: u32,
    pub(crate) retransmit_quota: u32,
}

impl BroadCastPolicy {
    pub fn get_retransmit_limit(&self) -> u32 {
        let scale = (self.node_num as f32 + 1.0).log10().ceil() as u32;
        self.retransmit_quota * scale
    }

    pub fn new(node_num: u32, retransmit_quota: u32) -> Self {
        Self {
            node_num,
            retransmit_quota,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node_group::gossip::broadcast::broadcast_manager_policy::BroadCastPolicy;

    #[test]
    fn test_broad_cast_policy() {
        let policy = BroadCastPolicy {
            node_num: 3,
            retransmit_quota: 3,
        };
        let value = policy.get_retransmit_limit();
        assert_eq!(value, 3);
    }
}
