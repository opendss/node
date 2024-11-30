use crate::common::selector::selector::{
    RandomSelector, RoundRobinSelector, Selector, SelectorFilterType,
};
use crate::common::time::backoff::{Backoff, ScaleBackoff};
use crate::node_group::gossip::goosip_state::State;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::options::Options;
use crate::node_group::{NodeStats, PingPacket};
use std::sync::{atomic, Arc};
use std::time::Duration;

pub struct ProbeTask<T>
where
    T: Transport,
{
    state: Arc<State>,
    options: Arc<Options>,
    transport: T,

    scaled_backoff: ScaleBackoff,
    round_robin_node_selector: RoundRobinSelector<NodeStats, SelectorFilterType>,
    random_node_selector: RandomSelector<NodeStats, SelectorFilterType>,
}
impl<T> ProbeTask<T>
where
    T: Transport,
{
    pub fn new(options: Arc<Options>, state: Arc<State>, transport: T) -> Self {
        Self {
            state,
            scaled_backoff: ScaleBackoff::new(Duration::from_secs(1), 10),
            round_robin_node_selector: RoundRobinSelector::new(Box::new({
                let current_node_id = options.as_ref().node_id.clone();
                move |x| x.node_id != current_node_id
            })),
            random_node_selector: RandomSelector::new(Box::new({
                let current_node_id = options.as_ref().node_id.clone();
                move |x| x.node_id != current_node_id
            })),
            options: options.clone(),
            transport,
        }
    }

    pub async fn run_probe(&mut self) {
        match self.round_robin_node_selector.next() {
            None => return,
            Some(candidate) => {
                let timeout = self.scaled_backoff.next_backoff();
                let host_address = self.options.bind_address.to_string();

                let ping_packet = PingPacket {
                    seq: self
                        .state
                        .sequence_generator
                        .fetch_add(1, atomic::Ordering::SeqCst),
                    source_node_id: self.options.node_id.clone(),
                    target_node_id: candidate.node_id.clone(),
                    target_address: Some(candidate.node_address.clone()),
                    source_address: host_address,
                };
                let pongPacket = self.transport.ping_unreliable_rpc(ping_packet).await;
            }
        }
    }
}
