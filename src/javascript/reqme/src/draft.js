import React, { Component } from "react";
import ReactDOM from "react-dom";
import Topbar from './topbar.js';
import { CSSTransition } from 'react-transition-group';
import { BrowserRouter as Router, Route, Link } from "react-router-dom";
import { getChampions, getGlobalRecommendations } from './requests.js';
import { SummonerSquare, ChampSquare } from './champions-squares.js';
import { MiddleContainer, TeamsContainer } from './smartdraft-container.js';
import {  ReqsContainer, Reqs, RoleToggle } from './reqs-container.js'; 
import ChampionPicker from './champion-picker.js';

const MAX_BANS = 10;
const MAX_PICKS = 5;



class Smartdraft extends React.Component {
    constructor() {
        super();
        this.state = {
            sameTeam: [],
            oppTeam: [],
            sameTeamIsRed: true,
            sameBans: [],
            oppBans: [],
            globalreqs: [],
            roles: ["Top", "Bottom", "Jungle", "Mid", "Support"],
            champions: [],
            mode: "manual", // enum - manual, websocket
            reqsloading: false
        };
        this.updateRoles = this.updateRoles;
    }

    componentDidMount() {
        getChampions().then(champions => {
            champions.sort((a,b) => {
                const x = a.name.toLowerCase();
                const y = b.name.toLowerCase();
                if (x < y) { return -1; }
                if (x > y) { return 1; }
                return 0;
            });
            this.setState({
                champions: champions
            });
        });
    }

    componentWillUnmount() {
        // close websocket
    }

    componentDidUpdate(prevProps, prevState) {
      if (this.state.roles.length !== prevState.roles.length) {
        this.getReqs();
      }
      if (this.state.mode === 'websocket' && prevState.mode === "manual") {
            // todo - move to separate file, have retry every 10s or manual button
            let tm = undefined;
            let webSocket = new WebSocket('ws://localhost:5000');
            webSocket.onmessage = event => {
                const teams = JSON.parse(event.data);
                this.setState({
                    sameTeam: teams.sameTeam,
                    oppTeam: teams.oppTeam
                });
                if (tm) {
                    tm.clearTimeout
                }
                tm = setTimeout(this.getReqs.bind(this), 500);
            }
        }
    }

    chooseDraftMode(mode) {
        this.setState({
            mode: mode
        });
    }

    addChampToSameTeam(champion) {
        let team = this.state.sameTeam;
        if (team.indexOf(champion.name) > -1 || team.length >= MAX_PICKS) {
            return;
        }
        return this.setState({
            sameTeam: team.concat([champion.name])
        });
    }

    addChampToOppTeam(champion) {
        let team = this.state.oppTeam;
        if (team.indexOf(champion.name) > -1 || team.length >= MAX_PICKS) {
            return;
        }
        return this.setState({
            oppTeam: team.concat([champion.name])
        });
    }

    removeChampFromSameTeam(champion) {
        let team = this.state.sameTeam;
        let champIdx = team.indexOf(champion.name)
        if (champIdx === -1 || !team.length) {
            return;
        }
        let newTeam = [];
        team.forEach(champName => {
            if (champName !== champion.name) {
                newTeam.push(champName);
            }
        })
        return this.setState({
            sameTeam: newTeam
        });
    }

     removeChampFromOppTeam(champion) {
        let team = this.state.oppTeam;
        let champIdx = team.indexOf(champion.name)
        if (champIdx === -1 || !team.length) {
            return;
        }
        let newTeam = [];
        team.forEach(champName => {
            if (champName !== champion.name) {
                newTeam.push(champName);
            }
        })
        return this.setState({
            oppTeam: newTeam
        });
    }

    getReqs() {
        this.setState({
            globalreqs: [],
            reqsloading: true
        });
        return getGlobalRecommendations(this.state.sameTeam, this.state.oppTeam, this.state.roles).then(recommendations => {
            return this.setState({
                globalreqs: recommendations.reqs,
                reqsloading: false
            });
        });

    }

    updateRoles(roles) {
        this.setState({
            roles: roles.map((r) => r.value)
        });
    }

    render() {
        let teamFns = {
            addChampToSameTeam: this.addChampToSameTeam.bind(this),
            addChampToOppTeam: this.addChampToOppTeam.bind(this),
            removeChampFromSameTeam: this.removeChampFromSameTeam.bind(this),
            removeChampFromOppTeam: this.removeChampFromOppTeam.bind(this)
        };
        let reqHelpers = {
            getReqs: this.getReqs.bind(this),
            updateRoles: this.updateRoles.bind(this),
            roles: this.state.roles,
            reqs: this.state.globalreqs,
            mode: this.state.mode,
            loading: this.state.reqsloading,
            chooseDraftMode: this.chooseDraftMode.bind(this)
        };
        return (
                <div className="app-container">
                    <MiddleContainer>
                        <ReqsContainer 
                            reqHelpers={reqHelpers}
                            allChampions={this.state.champions}
                        />
                        <TeamsContainer
                            teamFns={teamFns}
                            sameTeam={this.state.sameTeam} 
                            oppTeam={this.state.oppTeam}
                        />
                    </MiddleContainer>
                    {this.state.mode === 'manual' && 
                        <ChampionPicker 
                        teamFns={teamFns}
                        sameTeam={this.state.sameTeam} 
                        oppTeam={this.state.oppTeam}
                        champions={this.state.champions} 
                        />
                    }
                    
                </div>
            )
        }
}

function DraftView() {
    return (
        <div className="main-view-container">
            <Topbar />
            <Smartdraft />
        </div>
    );
}

export default DraftView;

