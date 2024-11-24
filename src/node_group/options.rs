use std::net::SocketAddr;

pub(crate) struct Options {
    pub(crate) node_id: String,
    pub(crate) bind_address: SocketAddr,
}
