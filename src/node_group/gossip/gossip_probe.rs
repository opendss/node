use std::time::Duration;
use crate::common::selector::selector::{RandomIndexSelector, RoundRobinIndexSelector};
use crate::common::time::backoff::ScaleBackoff;
use crate::node_group::gossip::gossip_state::State;
use crate::node_group::gossip::transport::Transport;
use crate::node_group::options::Options;

pub struct ProbeTask<T>
where
    T: Transport,
{
    state: State,
    options: Options,
    transport: T,
    scheduler: Scheduler,
}
struct Scheduler {
    scaled_backoff: ScaleBackoff,
    round_robin_node_selector: RoundRobinIndexSelector,
    random_node_selector: RandomIndexSelector,
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
            scheduler: Scheduler {
                scaled_backoff: ScaleBackoff::new(Duration::from_secs(1), 10),
                round_robin_node_selector: RoundRobinIndexSelector::new(10),
                random_node_selector: RandomIndexSelector::new(10),
            },
        }
    }

    pub async fn run_probe(&mut self) {
        // match self.round_robin_node_selector.next() {
        //     None => return,
        //     Some(candidate) => {
        //         let timeout = self.scaled_backoff.next_backoff();
        //         let host_address = self.options.bind_address.to_string();
        //
        //         let ping_packet = PingPacket {
        //             seq: self
        //                 .state
        //                 .sequence_generator
        //                 .fetch_add(1, atomic::Ordering::SeqCst),
        //             source_node_id: self.options.node_id.clone(),
        //             target_node_id: candidate.node_id.clone(),
        //             target_address: Some(candidate.node_address.clone()),
        //             source_address: host_address,
        //         };
        //         let ping_unreliable_rpc = self.transport.ping_unreliable_rpc(ping_packet);
        //         let check_result: Result<Option<PongPacket>, GossipError>;
        //         select! {
        //            pong_result = ping_unreliable_rpc => {
        //                 match pong_result {
        //                 Ok(pong) => check_result = Ok(Some(pong_result.unwrap())),
        //                 Err(err) => check_result = Err(err),
        //                 }
        //             }
        //             _ = sleep(self.options.probe_timeout) => {
        //                 check_result = Ok(None)
        //             }
        //         }
        //         if check_result.is_ok() {
        //             let option = check_result.as_ref().unwrap();
        //             if option.is_some() {
        //                 return;
        //             }
        //         } else {
        //             // todo: improve the log
        //             warn!(
        //                 "failed send probe by unreliable channel in the timeout. timeout={:?}",
        //                 self.options.probe_timeout
        //             )
        //         }
        //
        //         let nodes = self
        //             .random_node_selector
        //             .next(self.options.indirect_check_num);
        //     }
        // }
    }
}
