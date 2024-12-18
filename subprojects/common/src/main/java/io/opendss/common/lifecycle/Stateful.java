package io.opendss.common.lifecycle;

import io.opendss.common.api.Api;
import io.opendss.common.api.ApiVersion;

@Api(version = ApiVersion.ALPHA)
public interface Stateful {

    void start();

    void shutdown();

}
