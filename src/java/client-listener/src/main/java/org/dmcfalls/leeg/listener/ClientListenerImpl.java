package org.dmcfalls.leeg.listener;

import com.stirante.lolclient.ClientApi;
import com.stirante.lolclient.ClientWebSocket;
import org.dmcfalls.leeg.exceptions.InitializationException;
import org.dmcfalls.leeg.listener.socket.PickBanEventListener;

import java.io.Closeable;
import java.io.IOException;

public class ClientListenerImpl implements ClientListener, Closeable {

    private ClientWebSocket webSocket;

    @Override
    public void init() throws InitializationException {
        ClientApi clientApi = new ClientApi();
        String summonerName;
        try {
            summonerName = clientApi.getCurrentSummoner().displayName;
        } catch (IOException e) {
            System.out.println("Exception caught while initializing ClientListener clientApi: " + e.getMessage());
            e.printStackTrace();
            throw new InitializationException(e);
        }
        System.out.println("Summoner: " + summonerName);
        try {
            webSocket = clientApi.openWebSocket();
            webSocket.setSocketListener(new PickBanEventListener(summonerName));
            System.out.println("Listening for client events...");
        } catch (Exception e) {
            System.out.println("Exception caught while initializing ClientListener webSocket: " + e.getMessage());
            e.printStackTrace();
            throw new InitializationException(e);
        }
    }

    @Override
    public void close() {
        webSocket.close();
    }

}
