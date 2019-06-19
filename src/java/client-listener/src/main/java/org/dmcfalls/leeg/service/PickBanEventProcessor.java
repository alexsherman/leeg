package org.dmcfalls.leeg.service;

import com.stirante.lolclient.ClientWebSocket;
import generated.LolChampSelectChampSelectBannableChampions;
import generated.LolChampSelectChampSelectDisabledChampions;
import generated.LolChampSelectChampSelectPickableChampions;
import generated.LolChampSelectChampSelectPlayerSelection;
import generated.LolChampSelectChampSelectSession;
import org.dmcfalls.leeg.domain.ChampionSelectSnapshot;
import org.dmcfalls.leeg.domain.uri.PickBanUriEnum;

import java.util.HashSet;
import java.util.Optional;
import java.util.Set;

import static org.dmcfalls.leeg.converter.UriToEnumConverter.toPickBanUriEnum;

/**
 * Turns a client web event into a useful, serializable object
 * Maintains and updates state of champion select in order to produce a useful snapshot
 */
public class PickBanEventProcessor {

    private static final int NONE_CHAMPION_ID = 0;

    private String summonerName;

    private final Set<Integer> summonerTeamPartial = new HashSet<>();

    private final Set<Integer> opponentTeamPartial = new HashSet<>();

    private final Set<Integer> summonerTeamBansPartial = new HashSet<>();

    private final Set<Integer> opponentTeamBansPartial = new HashSet<>();

    /**
     * Builds an up-to-date snapshot of champion select after processing the supplied event
     * @param event a champion select event from the LoL client API
     * @return a snapshot containing information on the picks and bans so far if anything changed, otherwise Empty
     */
    public Optional<ChampionSelectSnapshot> buildChampionSelectSnapshot(ClientWebSocket.Event event) {
        boolean somethingChanged = processPickBanData(event);
        if (somethingChanged) {
            return Optional.of(generateSnapshot());
        } else {
            return Optional.empty();
        }
    }

    private boolean processPickBanData(ClientWebSocket.Event event) {
        PickBanUriEnum eventType = toPickBanUriEnum(event.getUri());
        switch (eventType) {
            case SESSION:
                return processSessionEvent(event.getData());
            case PICKABLE_CHAMPIONS:
                return processPickableChampionsEvent(event.getData());
            case BANNABLE_CHAMPIONS:
                return processBannableChampionsEvent(event.getData());
            case DISABLED_CHAMPIONS:
                return processDisabledChampionsEvent(event.getData());
            default:
                System.out.println("Unrecognized event in PickBanEventProcessor! Uri: " + event.getUri());
                return false;
        }
    }

    private boolean processSessionEvent(Object data) {
        boolean somethingChanged = false;
        if (data instanceof LolChampSelectChampSelectSession) {
            LolChampSelectChampSelectSession session = (LolChampSelectChampSelectSession) data;
            for (LolChampSelectChampSelectPlayerSelection selection : session.myTeam) {
                if (NONE_CHAMPION_ID == selection.championId) continue;
                somethingChanged |= summonerTeamPartial.add(selection.championId);
            }
            for (LolChampSelectChampSelectPlayerSelection selection : session.theirTeam) {
                if (NONE_CHAMPION_ID == selection.championId) continue;
                somethingChanged |= opponentTeamPartial.add(selection.championId);
            }
            somethingChanged |= summonerTeamBansPartial.addAll(session.bans.myTeamBans);
            somethingChanged |= opponentTeamBansPartial.addAll(session.bans.theirTeamBans);
        }
        return somethingChanged;
    }

    private boolean processPickableChampionsEvent(Object data) {
        if (data instanceof LolChampSelectChampSelectPickableChampions) {
            // TODO: implement me maybe
        }
        return false;
    }

    private boolean processBannableChampionsEvent(Object data) {
        if (data instanceof LolChampSelectChampSelectBannableChampions) {
            // TODO: implement me maybe
        }
        return false;
    }

    private boolean processDisabledChampionsEvent(Object data) {
        if (data instanceof LolChampSelectChampSelectDisabledChampions) {
            // TODO: implement me maybe
        }
        return false;
    }

    private ChampionSelectSnapshot generateSnapshot() {
        ChampionSelectSnapshot snapshot = new ChampionSelectSnapshot();
        snapshot.setSummonerName(summonerName);
        snapshot.setSummonerTeam(summonerTeamPartial);
        snapshot.setOpponentTeam(opponentTeamPartial);
        snapshot.setSummonerTeamBans(summonerTeamBansPartial);
        snapshot.setOpponentTeamBans(opponentTeamBansPartial);
        return snapshot;
    }

    public void setSummonerName(String summonerName) {
        this.summonerName = summonerName;
    }
}
