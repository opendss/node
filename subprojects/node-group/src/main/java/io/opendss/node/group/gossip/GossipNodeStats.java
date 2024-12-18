package io.opendss.node.group.gossip;

import io.opendss.common.protobuf.ProtobufEncode;
import io.opendss.node.group.NodeStats;

final class GossipNodeStats implements NodeStats, ProtobufEncode<io.github.opendss.node.proto.gossip.NodeStats> {

    @Override
    public io.github.opendss.node.proto.gossip.NodeStats encode() {
        return null;
    }
}
