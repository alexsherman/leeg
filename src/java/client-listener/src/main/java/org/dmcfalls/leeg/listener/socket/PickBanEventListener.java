package org.dmcfalls.leeg.listener.socket;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.stirante.lolclient.ClientWebSocket;
import org.dmcfalls.leeg.domain.ChampionSelectSnapshot;
import org.dmcfalls.leeg.filter.ChampSelectEventFilter;
import org.dmcfalls.leeg.filter.ClientEventFilter;
import org.dmcfalls.leeg.publisher.LocalWebSocketPublisher;
import org.dmcfalls.leeg.publisher.LocalWebSocketPublisherImpl;
import org.dmcfalls.leeg.service.PickBanEventProcessor;

import java.util.Optional;

public class PickBanEventListener implements ClientWebSocket.SocketListener {

    private final ClientEventFilter filter = new ChampSelectEventFilter();

    private final PickBanEventProcessor processor = new PickBanEventProcessor();

    private final LocalWebSocketPublisher publisher = new LocalWebSocketPublisherImpl();

    private final Gson gson = new GsonBuilder().create();

    public PickBanEventListener(String summonerName) {
        processor.setSummonerName(summonerName);
        publisher.open();
    }

    @Override
    public void onEvent(ClientWebSocket.Event event) {
        if (filter.allow(event)) {
            Optional<ChampionSelectSnapshot> result = processor.buildChampionSelectSnapshot(event);
            result.ifPresent(championSelectSnapshot -> publisher.publish(gson.toJson(championSelectSnapshot)));
        }
    }

    @Override
    public void onClose(int code, String reason) {
        System.out.println(this.getClass().getName() + " socket closed with code " + code
                + ", reason: " + reason);
        publisher.close();
    }

}
