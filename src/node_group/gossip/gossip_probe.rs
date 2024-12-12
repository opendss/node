use crate::common::selector::selector::{RandomIndexSelector, RoundRobinIndexSelector};
use crate::common::time::backoff::{Backoff, ScaleBackoff};
use crate::node_group::gossip::errors::GossipError;
use crate::node_group::gossip::gossip_state::State;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::options::Options;
use crate::node_group::{NodeStats, PingPacket, PongPacket};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::join;
use tokio::{join, select};
use tracing::{debug, info, warn};
use tracing_subscriber::fmt::time;

pub struct ProbeTask<T>
where
    T: Transport,
{
    state: State,
    options: Options,
    transport: T,
    scaled_timeout_backoff: ScaleBackoff,
}

impl<T> ProbeTask<T>
where
    T: Transport,
{
    pub fn new(options: Options, state: State, transport: T) -> Self {
        Self {
            state,
            options,
            transport,
            scaled_timeout_backoff: ScaleBackoff::new(Duration::from_secs(1), 10),
        }
    }

    pub async fn run_probe(&mut self) {
        let candidate: Arc<NodeStats>;
        loop {
            let node_option = self.state.round_robin_next().await;
            if node_option.is_none() {
                // todo: avoid busy loop
                continue;
            }
            candidate = node_option.unwrap();
            break;
        }
        let scaled_timeout = self.scaled_timeout_backoff.next_backoff();
        let host_address = self.options.bind_address().to_string();
        let node_id = self.options.node_id();

        let ping_packet = PingPacket {
            seq: self.state.next_seq(),
            source_node_id: node_id.to_string(),
            source_address: host_address,
            target_node_id: candidate.node_id.clone(),
            target_address: None,
        };
        let target_address = SocketAddr::from_str(candidate.node_address.as_str()).unwrap();
        let unreliable_ping_result = self
            .transport
            .ping_unreliable_rpc(
                target_address,
                ping_packet.clone(),
                self.options.probe_timeout(),
            )
            .await;
        if unreliable_ping_result.is_ok() {
            return;
        }
        warn!(
            "unreliable ping failed. error:{:?}",
            unreliable_ping_result.err().unwrap()
        );
        let mut indirect_packages = Vec::new();
        let random_nodes = self
            .state
            .random_nodes(self.options.indirect_check_num())
            .await;
        for candidate in random_nodes {
            let mut indirect_packet = ping_packet.clone();
            indirect_packet.target_address = Some(candidate.node_address.clone());
            indirect_packet.target_node_id = candidate.node_id.clone();
            let addr =
                SocketAddr::from_str(indirect_packet.target_address.clone().unwrap().as_str())
                    .unwrap();
            let future =
                self.transport
                    .ping_unreliable_rpc(addr, indirect_packet, Duration::from_secs(1));
            indirect_packages.push(future);
        }

        let result_vec = futures::future::join_all(indirect_packages).await;
        if result_vec.iter().any(|r| r.is_ok()) {
            return;
        }
        warn!("unreliable indirect ping failed. errors:{:?}", result_vec);
    }
}
