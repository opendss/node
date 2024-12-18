package io.opendss.common.protobuf;

import com.google.protobuf.GeneratedMessage;

public interface ProtobufEncode<T extends GeneratedMessage> {

    T encode();
}
