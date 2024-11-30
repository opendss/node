use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::{PingPacket, PongPacket};

pub trait Transport : Stateful{
    async fn ping_unreliable_rpc(
        &mut self,
        ping_packet: PingPacket,
    ) -> Result<PongPacket, GossipError>;
}
