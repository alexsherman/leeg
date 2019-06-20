package org.dmcfalls.leeg.publisher;

public interface LocalWebSocketPublisher {

    /**
     * Open connection to a local web-socket endpoint
     */
    void open();

    /**
     * Push a message to the endpoint
     */
    void publish(String payload);

    /**
     * Close connection
     */
    void close();

}
