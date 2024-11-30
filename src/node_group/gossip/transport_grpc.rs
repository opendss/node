use std::cell::RefCell;
use crate::common::errors::CommonError;
use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::gossip::transport_handler::PacketHandler;
use crate::node_group::gossip_client::GossipClient;
use crate::node_group::gossip_server::{Gossip, GossipServer};
use crate::node_group::{Packet, PingPacket, PongPacket, PushPull};
use dashmap::DashMap;
use prost::Message;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};
use tracing::error;

#[derive(Clone)]
pub struct GrpcTransport {
    bind_address: SocketAddr,
    stub_pool: Arc<DashMap<String, Option<GossipClient<Channel>>>>,
    unreliable: Option<Arc<UdpSocket>>,
    packet_handler: Arc<PacketHandler>,
    transport_handlers: Arc<Mutex<GrpcTransportHandle>>,
}

struct GrpcTransportHandle {
    unreliable_handle: Option<JoinHandle<()>>,
    grpc_server_handle: Option<JoinHandle<()>>,
}

impl GrpcTransport {
    pub fn new(bind_address: SocketAddr) -> Self {
        Self {
            unreliable: None,
            packet_handler: Arc::new(()),
            transport_handlers: Arc::new(Mutex::new(GrpcTransportHandle{
                unreliable_handle: None,
                grpc_server_handle: None,
            })),
            bind_address,
            stub_pool: Arc::new(DashMap::new()),
        }
    }
}

impl Transport for GrpcTransport {
    async fn ping_unreliable_rpc(
        &mut self,
        ping_packet: PingPacket,
    ) -> Result<PongPacket, GossipError> {
        todo!()
    }
}

impl Stateful for GrpcTransport {
    async fn start(&mut self) -> Result<(), CommonError> {
        let addr = self.bind_address.clone();
        self.unreliable = Some(Arc::new(UdpSocket::bind(self.bind_address).await?));

        let unreliable = self.unreliable.as_ref().unwrap().clone();
        let handler = self.packet_handler.clone();
        let mut guard = self.transport_handlers.lock().await;
        guard.unreliable_handle = Some(tokio::spawn(async move {
            loop {
                let mut buf = [0u8; 1024];
                let result = unreliable.recv(buf.as_mut()).await;
                if result.is_err() {
                    error!(
                        "failed to receive message from unreliable channel. error={}",
                        result.unwrap_err()
                    );
                    std::process::exit(1);
                }
                let actual_size = result.unwrap();
                if actual_size >= 0 {
                    let decode_result = Packet::decode(&buf[0..actual_size]);
                    if decode_result.is_err() {
                        error!(
                            "failed to decode the message. error={}",
                            decode_result.unwrap_err()
                        );
                        continue;
                    }
                    let packet = decode_result.unwrap();
                    let clone_handler = handler.clone();
                    tokio::spawn(async move {
                        clone_handler.handle_packet(packet).await;
                    });
                }
            }
        }));

        let ph = self.packet_handler.clone();
        guard.grpc_server_handle = Some(tokio::spawn(async move {
            let server = GossipServer::new(GossipService::new(ph));
            let result = Server::builder().add_service(server).serve(addr).await;
            if result.is_err() {
                error!(
                    "failed to start gossip reliable server. error={}",
                    result.err().unwrap()
                );
                std::process::exit(1);
            }
        }));
        // client example
        Ok(())
    }

    async fn close(&mut self) -> Result<(), CommonError> {
        todo!()
    }
}

struct GossipService {
    packet_handler: Arc<PacketHandler>,
}

impl GossipService {
    pub fn new(packet_handler: Arc<PacketHandler>) -> Self {
        Self { packet_handler }
    }
}

#[tonic::async_trait]
impl Gossip for GossipService {
    async fn push_pull(&self, request: Request<PushPull>) -> Result<Response<PushPull>, Status> {
        self.packet_handler.handle_push_pull(request).await
    }

    async fn ping(&self, request: Request<PingPacket>) -> Result<Response<PongPacket>, Status> {
        self.packet_handler.handle_ping_request(request).await
    }
}
