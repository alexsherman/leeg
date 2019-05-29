package org.dmcfalls.leeg.domain;

import java.io.Serializable;
import java.util.List;
import java.util.Set;

/**
 * General data structure to hold useful information about champion select that we want to publish
 */
public class ChampionSelectSnapshot implements Serializable {

    private String summonerName;

    private Set<Integer> summonerTeam;

    private Set<Integer> opponentTeam;

    private Set<Integer> summonerTeamBans;

    private Set<Integer> opponentTeamBans;

    /*
     TODO: Decide if the below are worth implementing and implement if so
       * cellId and summonerId for each champion selection
     */

    public String getSummonerName() {
        return summonerName;
    }

    public void setSummonerName(String summonerName) {
        this.summonerName = summonerName;
    }

    public Set<Integer> getSummonerTeam() {
        return summonerTeam;
    }

    public void setSummonerTeam(Set<Integer> summonerTeam) {
        this.summonerTeam = summonerTeam;
    }

    public Set<Integer> getOpponentTeam() {
        return opponentTeam;
    }

    public void setOpponentTeam(Set<Integer> opponentTeam) {
        this.opponentTeam = opponentTeam;
    }

    public Set<Integer> getSummonerTeamBans() {
        return summonerTeamBans;
    }

    public void setSummonerTeamBans(Set<Integer> summonerTeamBans) {
        this.summonerTeamBans = summonerTeamBans;
    }

    public Set<Integer> getOpponentTeamBans() {
        return opponentTeamBans;
    }

    public void setOpponentTeamBans(Set<Integer> opponentTeamBans) {
        this.opponentTeamBans = opponentTeamBans;
    }

}
