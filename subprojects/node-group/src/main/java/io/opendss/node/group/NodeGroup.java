package io.opendss.node.group;

import io.opendss.common.api.Api;
import io.opendss.common.api.ApiVersion;
import io.opendss.common.lifecycle.Stateful;

@Api(version = ApiVersion.ALPHA)
public interface NodeGroup extends Stateful {
}
