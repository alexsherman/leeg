package org.dmcfalls.leeg.publisher;

import java.net.URI;

public class LocalWebSocketPublisherImpl implements LocalWebSocketPublisher {

    private static final String LOCAL_ADDRESS = "localhost:8080/client-listener/";

    private LocalWebSocketEndpoint localEndpoint;

    @Override
    public void open() {
        URI localAddressUri = URI.create(LOCAL_ADDRESS);
        localEndpoint = new LocalWebSocketEndpoint(localAddressUri);
    }

    @Override
    public void publish(String payload) {
        publishToLocalWebSocket(payload);
        publishToLog(payload);
    }

    @Override
    public void close() {
        localEndpoint = null;
    }

    private void publishToLocalWebSocket(String payload) {
        localEndpoint.sendMessage(payload);
    }

    private void publishToLog(String payload) {
        System.out.println("Publishing a message: '" + payload + "'");
    }

}
