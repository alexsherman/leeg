package org.dmcfalls.leeg.listener;

import org.dmcfalls.leeg.exceptions.InitializationException;

/**
 * Listens to events from the league of legends client.
 * Wraps around the ClientApi and ClientWebSocket from the stirante library
 */
public interface ClientListener {

    /**
     * Begins listening to the client
     * @throws InitializationException when an error occurs during initialization
     */
    void init() throws InitializationException;

    /**
     * Releases any resources. Should be called when the ClientListener is no longer needed
     */
    void close();

}
