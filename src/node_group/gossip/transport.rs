use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip_server::{Gossip, GossipServer};
use crate::node_group::{PingPacket, PongPacket, PushPull};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};
use crate::node_group::gossip_client::GossipClient;

struct Transport {
    bind_address: SocketAddr,
    unreliable: Option<UdpSocket>,
}

impl Transport {
    pub fn new(bind_address: SocketAddr) -> Self {
        Self {
            unreliable: None,
            bind_address,
        }
    }

    pub async fn send_unreliable() -> Result<(), ()> {
        ()
    }

    pub async fn rpc() -> Result<(), ()> {}
}

impl Stateful for Transport {
    async fn start(&mut self) {
        self.unreliable = Some(UdpSocket::bind(self.bind_address).await.unwrap());

        let server = GossipServer::new(GossipService::default());
        let x = Server::builder()
            .add_service(server)
            .serve(self.bind_address)
            .await;


        let endpoint = Channel::from_static("").connect().await.unwrap();
        let client = GossipClient::new(endpoint);
        // client example
    }

    async fn close(&mut self) {
        todo!()
    }
}

#[derive(Default)]
struct GossipService {}

#[tonic::async_trait]
impl Gossip for GossipService {
    async fn push_pull(&self, request: Request<PushPull>) -> Result<Response<PushPull>, Status> {
        todo!()
    }

    async fn ping(&self, request: Request<PingPacket>) -> Result<Response<PongPacket>, Status> {
        todo!()
    }
}
