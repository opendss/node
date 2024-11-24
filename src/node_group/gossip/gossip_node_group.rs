use crate::common::lifecycle::stateful::Stateful;
use crate::common::selector::selector::{RandomSelector, RoundRobinSelector, Selector};
use crate::common::time::backoff::{Backoff, ScaleBackoff};
use crate::node_group::node_group::NodeGroup;
use crate::node_group::options::Options;
use crate::node_group::{NodeStats, PingPacket, PongPacket};
use dashmap::DashMap;
use std::io::Error;
use std::sync::{atomic, Arc};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;
use tracing::{error, warn};

pub(crate) struct GossipNodeGroup {
    option: Arc<Options>,
    state: Arc<State>,
    probe_handle: Option<JoinHandle<()>>,
}

struct ProbeTask<F>
where
    F: Fn(&NodeStats) -> bool + Send + Sync,
{
    state: Arc<State>,
    options: Arc<Options>,

    scaled_backoff: ScaleBackoff,
    round_robin_node_selector: RoundRobinSelector<NodeStats, F>,
    random_node_selector: RandomSelector<NodeStats, F>,
}
impl<F> ProbeTask<F>
where
    F: Fn(&NodeStats) -> bool + Send + Sync + Clone,
{
    pub fn new(options: Arc<Options>, state: Arc<State>, filter: F) -> Self {
        Self {
            state,
            options: options.clone(),
            scaled_backoff: ScaleBackoff::new(Duration::from_secs(1), 10),
            round_robin_node_selector: RoundRobinSelector::new(filter.clone()),
            random_node_selector: RandomSelector::new(filter),
        }
    }

    async fn run_probe(&mut self) {
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
                let result = self.unreliable_ping(ping_packet, &timeout).await;
            }
        }
    }
}

struct State {
    sequence_generator: atomic::AtomicU64,
    nodes: DashMap<String, NodeStats>,
}
impl State {
    pub fn new() -> Self {
        Self {
            sequence_generator: atomic::AtomicU64::new(0),
            nodes: DashMap::new(),
        }
    }
}

impl GossipNodeGroup {
    pub fn new(option: Options) -> Self {
        let shard_option = Arc::new(option);
        let state = Arc::new(State::new());
        Self {
            option: shard_option.clone(),
            state: state.clone(),
            probe_handle: None,
        }
    }

    async fn unreliable_ping(
        &mut self,
        ping: PingPacket,
        timeout: &Duration,
    ) -> Result<PongPacket, Error> {
    }
}

impl Stateful for GossipNodeGroup {
    async fn start(&mut self) {
        let option = self.option.clone();
        let state = self.state.clone();
        self.probe_handle = Some(tokio::spawn(async move {
            let mut task =
                ProbeTask::new(option.clone(), state, move |x| x.node_id == option.node_id);
            let mut probe_interval = time::interval(Duration::from_secs(1));

            loop {
                probe_interval.tick().await;

                task.run_probe().await;
            }
            ()
        }));
    }

    async fn close(&mut self) {
        if self.probe_handle.is_some() {
            self.probe_handle.as_ref().unwrap().abort();
            match self.probe_handle.as_mut().unwrap().await {
                Ok(_) => {}
                Err(err) => {
                    warn!("close probe handle task error. error={}", err)
                }
            }
        }
    }
}

impl NodeGroup for GossipNodeGroup {}
