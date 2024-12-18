package io.opendss.node.group.gossip;

import io.opendss.node.group.NodeGroup;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

final class GossipNodeGroup implements NodeGroup {
    private final Map<String, GossipNodeStats> stats;

    GossipNodeGroup() {
        this.stats = new ConcurrentHashMap<>();
    }

    @Override
    public void start() {

    }

    @Override
    public void shutdown() {

    }
}
