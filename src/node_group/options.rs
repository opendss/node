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

impl Options {
    pub fn probe_timeout(&self) -> Duration {
        self.inner.probe_timeout
    }
    pub fn bind_address(&self) -> SocketAddr {
        self.inner.bind_address
    }

    pub fn node_id(&self) -> &str {
        self.inner.node_id.as_str()
    }
    
    pub fn indirect_check_num(&self) -> u32 {
        self.inner.indirect_check_num
    }
}
