use crate::common::selector::selector::{RandomIndexSelector, RoundRobinIndexSelector};
use crate::common::time::backoff::{Backoff, ScaleBackoff};
use crate::node_group::gossip::gossip_state::State;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::options::Options;
use crate::node_group::{NodeStats, PingPacket};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use rand::random;
use tokio::select;
use tracing::{debug, info, warn};

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
        let backoff = self.scaled_timeout_backoff.next_backoff();
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
        let pong_packet_future =
            self.transport
                .ping_unreliable_rpc(target_address, ping_packet.clone(), backoff);
        let timeout = tokio::time::sleep(self.options.probe_timeout());
        select! {
            result = pong_packet_future => {
                if let Ok(_) = result {
                    return
                }
                let err = result.err().unwrap()
                warn!("receive error when send ping through unreliable channel. {:?}", err)
            }
            _ = timeout => {
                warn!("timeout when send ping through unreliable channel.")
            }
        }
        let random_nodes = self.state.random_nodes(self.options.indirect_check_num()).await;
        let indirect_packet = ping_packet.clone();

    }
}
