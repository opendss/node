use std::sync::Arc;
use crate::node_group::gossip::gossip_state::State;

#[derive(Clone)]
struct PacketProcessor {
    inner: Arc<Inner>
}
struct Inner {
    state: State
}

impl PacketProcessor {

    
}