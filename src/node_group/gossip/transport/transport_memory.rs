use crate::common::errors::CommonError;
use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::{PingPacket, PongPacket};
use tonic::async_trait;

struct MemoryTransport {}

#[async_trait]
impl Stateful for MemoryTransport {
    async fn start(&mut self) -> Result<(), CommonError> {
        todo!()
    }

    async fn close(&mut self) -> Result<(), CommonError> {
        todo!()
    }
}

impl Clone for MemoryTransport {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[async_trait]
impl Transport for MemoryTransport {
    async fn ping_unreliable_rpc(
        &mut self,
        ping_packet: PingPacket,
    ) -> Result<PongPacket, GossipError> {
        todo!()
    }
}
