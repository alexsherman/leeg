package org.dmcfalls.leeg.publisher;

import javax.websocket.ClientEndpoint;
import javax.websocket.CloseReason;
import javax.websocket.ContainerProvider;
import javax.websocket.OnClose;
import javax.websocket.OnError;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.WebSocketContainer;
import java.io.IOException;
import java.net.URI;

@ClientEndpoint
public class LocalWebSocketEndpoint  {

    private Session session = null;

    LocalWebSocketEndpoint(URI endpointURI) {
        try {
            WebSocketContainer container = ContainerProvider.getWebSocketContainer();
            container.connectToServer(this, endpointURI);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * Publish a message to the local websocket
     *
     * @param message the message to send
     */
    void sendMessage(String message) {
        this.session.getAsyncRemote().sendText(message);
    }

    /**
     * Callback hook for connection open events
     *
     * @param session the session which is opened
     */
    @OnOpen
    public void onOpen(Session session) {
        System.out.println("Opening websocket");
        this.session = session;
    }

    /**
     * Callback hook for connection close events
     *
     * @param session the session which is getting closed.
     * @param reason the reason for connection close
     */
    @OnClose
    public void onClose(Session session, CloseReason reason) {
        System.out.println("Closing websocket: " + reason);
        try {
            this.session.close();
        } catch (IOException e) {
            // Do nothing
        } finally {
            this.session = null;
        }
    }

    /**
     * Callback hook for errors during execution
     * @param t the error thrown
     */
    @OnError
    public void onError(Throwable t) {
        System.out.println("Error during processing: " + t.toString());
        t.printStackTrace();
    }

}
