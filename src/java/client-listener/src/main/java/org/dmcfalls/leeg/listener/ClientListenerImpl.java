package org.dmcfalls.leeg.listener;

import com.stirante.lolclient.ClientApi;
import com.stirante.lolclient.ClientConnectionListener;
import com.stirante.lolclient.ClientWebSocket;
import generated.LolSummonerSummoner;
import org.dmcfalls.leeg.exceptions.InitializationException;
import org.dmcfalls.leeg.listener.socket.PickBanEventListener;

import java.io.Closeable;
import java.io.IOException;

public class ClientListenerImpl implements ClientListener, Closeable {

    private static final String SUMMONER_URI = "/lol-summoner/v1/current-summoner";

    private ClientApi clientApi;

    private ClientWebSocket webSocket;

    @Override
    public void init() throws InitializationException {
        clientApi = new ClientApi();
        clientApi.addClientConnectionListener(new ClientConnectionListener() {

            @Override
            public void onClientConnected() {
                System.out.println("Client API connected to LoL client");
                connectAndAddListener();
            }

            @Override
            public void onClientDisconnected() {
                System.out.println("Client API disconnected!");
            }

        });
    }

    @Override
    public void close() {
        webSocket.close();
    }

    private void connectAndAddListener() {
        String summonerName;
        try {
            summonerName = clientApi.executeGet(SUMMONER_URI, LolSummonerSummoner.class).displayName;
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

}
