use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::{PingPacket, PongPacket};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::async_trait;

mod transport_grpc;
pub mod transport_memory;

#[async_trait]
pub trait Transport: Stateful + Clone + Send + 'static {
    async fn ping_unreliable_rpc(
        &mut self,
        target_node: SocketAddr,
        ping_packet: PingPacket,
        timeout: Duration,
    ) -> Result<PongPacket, GossipError>;
}
