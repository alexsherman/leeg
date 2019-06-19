package org.dmcfalls.leeg.filter;

import com.stirante.lolclient.ClientWebSocket;

public interface ClientEventFilter {

    /**
     * Invokes the filter on the given event
     * @return true if the event should be allowed through the filter, false otherwise
     */
    boolean allow(ClientWebSocket.Event event);

}
