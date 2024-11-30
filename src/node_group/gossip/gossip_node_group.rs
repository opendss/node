use crate::common::errors::CommonError;
use crate::common::lifecycle::stateful::Stateful;
use crate::common::selector::selector::{
    RandomSelector, RoundRobinSelector, Selector, SelectorFilterType,
};
use crate::common::time::backoff::{Backoff, ScaleBackoff};
use crate::node_group::gossip::gossip_state::State;
use crate::node_group::gossip::gossip_probe::ProbeTask;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::node_group::NodeGroup;
use crate::node_group::options::Options;
use crate::node_group::{NodeStats, PingPacket, PongPacket};
use dashmap::DashMap;
use std::io::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{atomic, Arc, RwLock};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;
use tonic::client::Grpc;
use tracing::warn;

pub(crate) struct GossipNodeGroup<T>
where
    T: Transport,
{
    option: Arc<Options>,
    state: Arc<State>,
    probe_handle: Option<JoinHandle<()>>,
    transport: T,
}

impl<T> GossipNodeGroup<T>
where
    T: Transport,
{
    pub fn new(option: Options, transport: T) -> Self {
        let shard_option = Arc::new(option);
        let state = Arc::new(State::new());
        Self {
            option: shard_option.clone(),
            state: state.clone(),
            probe_handle: None,
            transport,
        }
    }
}

impl<T> Stateful for GossipNodeGroup<T>
where
    T: Transport,
{
    async fn start(&mut self) -> Result<(), CommonError> {
        let option = self.option.clone();
        let state = self.state.clone();
        self.transport.start().await?;
        let transport = self.transport.clone();
        self.probe_handle = Some(tokio::spawn(async move {
            let mut task = ProbeTask::new(option, state, transport);
            let mut probe_interval = time::interval(Duration::from_secs(1));

            loop {
                probe_interval.tick().await;

                task.run_probe().await;
            }
        }));
        Ok(())
    }

    async fn close(&mut self) -> Result<(), CommonError> {
        if self.probe_handle.is_some() {
            self.probe_handle.as_ref().unwrap().abort();
            match self.probe_handle.as_mut().unwrap().await {
                Ok(_) => {}
                Err(err) => {
                    warn!("close probe handle task error. error={}", err)
                }
            }
        }
        Ok(())
    }
}
