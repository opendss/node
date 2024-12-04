use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub(crate) struct Options {
    inner: Arc<Inner>,
}

struct Inner {
    node_id: String,
    bind_address: SocketAddr,
    probe_timeout: Duration,
    indirect_check_num: u32,
}

impl Inner {
    pub fn new(
        node_id: String,
        bind_address: SocketAddr,
        probe_timeout: Duration,
        indirect_check_num: u32,
    ) -> Self {
        Self {
            node_id,
            bind_address,
            probe_timeout,
            indirect_check_num,
        }
    }
}
