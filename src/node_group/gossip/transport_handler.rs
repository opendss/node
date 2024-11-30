use crate::node_group::gossip::gossip_node_group::State;
use crate::node_group::{Packet, PingPacket, PongPacket, PushPull};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct PacketHandler {
    state: Arc<State>,
}

impl PacketHandler {
    pub async fn handle_packet(&self, packet: Packet) {}

    #[inline]
    pub async fn handle_push_pull(
        &self,
        push_pull: Request<PushPull>,
    ) -> Result<Response<PushPull>, Status> {
    }

    #[inline]
    pub async fn handle_ping_request(
        &self,
        request: Request<PingPacket>,
    ) -> Result<Response<PongPacket>, Status> {
    }

    pub fn new() -> Self {
        Self {}
    }
}
