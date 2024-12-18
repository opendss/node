package io.opendss.node.group.gossip;

import io.github.opendss.node.proto.gossip.NodeStatus;
import io.opendss.common.protobuf.ProtobufEncode;
import io.opendss.node.group.NodeStats;
import lombok.Data;

import java.net.SocketAddress;

@Data
final class GossipNodeStats implements NodeStats, ProtobufEncode<io.github.opendss.node.proto.gossip.NodeStats> {
    private final String nodeId;
    private final SocketAddress nodeAddress;
    private final GossipNodeMetadata metadata;
    private long term;
    private io.github.opendss.node.proto.gossip.NodeStatus status;

    GossipNodeStats(String nodeId, SocketAddress address) {
        this.nodeId = nodeId;
        this.nodeAddress = address;
        this.metadata = new GossipNodeMetadata();
        this.term = 0;
        this.status = NodeStatus.UNRECOGNIZED;
    }

    @Override
    public io.github.opendss.node.proto.gossip.NodeStats encode() {
        return null;
    }
}
