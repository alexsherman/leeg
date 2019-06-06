import React, { Component } from "react";
import ReactDOM from "react-dom";
import Select from 'react-select';
import Sidebar from './sidebar.js';
import { CSSTransition } from 'react-transition-group';
import { BrowserRouter as Router, Route, Link } from "react-router-dom";

function RoleToggles(props) {
    const options = [
        {"value": "Top", "label": "Top"},
        {"value": "Jungle", "label": "Jungle"},
        {"value": "Mid", "label": "Mid"},
        {"value": "Bottom", "label": "Bottom"},
        {"value": "Support", "label": "Support"},
    ];
    const placeholder = "Select one or more roles to filter recommendations."
    return (
        <Select
        className="role-select"
        isMulti
        placeholder={placeholder}
        onChange={props.updateRoles}
        options={options}
      />
    )
}

function SummonerSquare(props) {
    const champ = props.champion;
    const rank = (props.idx !== undefined) ? "#" + (props.idx + 1) + " - " : "";
    return (
        <div className="summoner-square">
            <ChampSquare champion={champ} />
            <div className="champion-name">{rank}{champ}</div>
        </div>
    )
}

export function ChampSquare(props) {
    let name = props.champion;
    name = name.split(' ').join("").split("'").join("").split('.').join("");
    if (name === "Wukong") {
        name = "MonkeyKing";
    }
    if (name === "LeBlanc") {
        name = "Leblanc";
    }
    if (name === "KaiSa") {
        name = "Kaisa";
    }
    if (name === "KhaZix") {
        name = "Khazix";
    }
    if (name === "VelKoz") {
        name = "Velkoz";
    }
    if (name === "ChoGath") {
        name = "Chogath";
    }

    const src = "url(http://ddragon.leagueoflegends.com/cdn/9.10.1/img/champion/" + name + ".png)";
    const style = {"backgroundImage": src, "backgroundSize": "cover", "backgroundPosition": "center"};
    return (
        <div className="champion-square" style={style}></div>
    )
}

function TeamLabel(props) {
    return (
        <div className="team-label">
            {props.label}
        </div>
    )
}

function Team(props) {
    const champs = props.teamdata.champs;
    const label = props.label;
    const summonerSquares = champs.map((champ) => 
        <SummonerSquare key={champ} champion={champ} />
    );
    return (
        <div className={"team-container " + props.team}>
            <TeamLabel label={label} />
            <CSSTransition in={inProp} timeout={200} classNames="selections">
            {summonerSquares}
            </CSSTransition>
        </div>
    );
}

function Reqs(props) {
    if (props.resp === null) {
        return;
    }
    const reqs = props.resp;
    const indivReqs = reqs.map((champ, idx) => 
        <React.Fragment>
            <SummonerSquare idx={idx} key={champ} champion={champ} />
        </React.Fragment>
    );
    return (
        <div className="center-container">
            <RoleToggles roles={props.roles} updateRoles={props.updateRoles} />
            <div className="req-container">
                {indivReqs}            
            </div>
        </div>
    );
}

class ChampionSelect extends React.Component {
    constructor() {
        super();
        this.state = {
            sameTeam: {
                    champs: [
                        "Ahri", "Ezreal",
                    ]
                },
            oppTeam: {
                    champs: [
                        "Riven"
                    ]
            },
            req: [],
            roles: ["Top", "Bottom", "Jungle", "Mid", "Support"]
        }
        this.updateRoles = this.updateRoles.bind(this);
    }

    componentDidMount() {
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

    componentWillUnmount() {
        // close websocket
    }

    componentDidUpdate(prevProps, prevState) {
      if (this.state.roles.length !== prevState.roles.length) {
        this.getReqs();
      }
    }

    getReqs() {

        // todo - move to separate file to handle all recommendation API call logic
        const baseUrl = 'http://localhost:8000/globalreq';
        let params = '?';
        if (this.state.sameTeam.champs.length > 0) {
            params += 'team=' + this.state.sameTeam.champs.join(',');
        }
        if (this.state.oppTeam.champs.length > 0) {
            params += '&opp=' + this.state.oppTeam.champs.join(',');
        }
        if (this.state.oppTeam.champs.length === 0 && this.state.sameTeam.champs.length === 0) {
            return;
        }

        if (this.state.roles.length) {
            params += "&roles=" + this.state.roles.join(",");
        }

        fetch(baseUrl + params, {
            method: "GET",
            mode: "cors",

            headers: {
                "Accept": "application/json"
            }
        }).then(resp => {

            resp.json().then(text => {
                this.setState({
                    req: text.reqs
                });
            }); 
        }).catch(err => {
            console.log(err);
        })
    }

    updateRoles(roles) {
        this.setState({
            roles: roles.map((r) => r.value)
        });
    }

    render() {
        return (
                <div className="app-container">
                    <Team team={"blue-team"} teamdata={this.state.sameTeam} label="Your Team" />
                    <Reqs resp={this.state.req} roles={this.state.roles} updateRoles={this.updateRoles} />
                    <Team team={"red-team"} teamdata={this.state.oppTeam} label="Enemy Team"/>
                </div>
            )
        }
}

function DraftView() {
    return (
        <div className="main-view-container">
            <Sidebar />
            <ChampionSelect />
        </div>
    );
}

export default DraftView;

