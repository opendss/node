package io.opendss.node.group.gossip;

import io.github.opendss.node.proto.gossip.NodeMetadata;
import io.opendss.common.protobuf.ProtobufEncode;

final class GossipNodeMetadata implements ProtobufEncode<NodeMetadata> {
    @Override
    public NodeMetadata encode() {
        return NodeMetadata.newBuilder().build();
    }
}
