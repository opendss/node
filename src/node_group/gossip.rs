use crate::common::stateful::Stateful;
use crate::node_group::node_group::NodeGroup;
use crate::node_group::options::Options;

struct GossipNodeGroup {
    option: Options,
}

impl Stateful for GossipNodeGroup {
    async fn start() {
        todo!()
    }

    fn close() {
        todo!()
    }
}

impl NodeGroup for GossipNodeGroup {
    
}