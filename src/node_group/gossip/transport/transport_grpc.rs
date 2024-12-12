use crate::common::errors::CommonError;
use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip::broadcast::broadcast_manager::BroadCastManager;
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::gossip_server::Gossip;
use crate::node_group::options::Options;
use crate::node_group::{Packet, PingPacket, PushPull};
use std::sync::Arc;
use std::time::Duration;
use prost::{bytes, Message};
use tokio::net::UdpSocket;
use tonic::{async_trait, Request, Response, Status};

#[derive(Clone)]
struct GrpcTransport {
    option: Options,
    inner: Arc<Inner>,
}

struct Inner {
    broadcast_manager: BroadCastManager,
}

#[async_trait]
impl Stateful for GrpcTransport {
    async fn start(&mut self) -> Result<(), CommonError> {
        let addr = self.option.bind_address();
        let socket = Arc::new(UdpSocket::bind(addr).await?);
        tokio::spawn(async move {
            let mut buf = [0; 65507];
            let result = socket.recv_from(&mut buf).await;
            match result {
                Ok(context) => {
                    let decode_result = Packet::decode(&buf[..context.0]);
                    if decode_result.is_err() {  }
                }
                Err(_) => {}
            }
        });

        todo!()
    }

    async fn close(&mut self) -> Result<(), CommonError> {
        todo!()
    }
}

#[async_trait]
impl Transport for GrpcTransport {
    async fn ping_unreliable_rpc(
        &self,
        target_node: SocketAddr,
        ping_packet: PingPacket,
        timeout: Duration,
    ) -> Result<PongPacket, GossipError> {
        todo!()
    }

    async fn ping_reliable_rpc(
        &self,
        target_node: SocketAddr,
        ping_packet: PingPacket,
        timeout: Duration,
    ) -> Result<PongPacket, GossipError> {
        todo!()
    }
}

#[async_trait]
impl Gossip for GrpcTransport {
    async fn push_pull(&self, request: Request<PushPull>) -> Result<Response<PushPull>, Status> {
        todo!()
    }

    async fn ping(&self, request: Request<PingPacket>) -> Result<Response<PongPacket>, Status> {
        todo!()
    }
}
