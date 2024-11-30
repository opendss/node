use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::{PingPacket, PongPacket};
use tonic::async_trait;

pub mod transport_memory;
mod transport_grpc;

#[async_trait]
pub trait Transport: Stateful + Clone + Send + 'static {
    async fn ping_unreliable_rpc(
        &mut self,
        ping_packet: PingPacket,
    ) -> Result<PongPacket, GossipError>;
}
