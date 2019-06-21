import Team from './team.js';
import React, { Component } from "react";
import ReactDOM from "react-dom";

export function MiddleContainer(props) {
    return (
        <div className='middle-container'>
            {props.children}
        </div>
    )
}

export function ReqsContainer(props) {
    return (
        <div className='reqs-container'>
            {props.children}
        </div>
    )
}

export function TeamsContainer(props) {
    let sameTeam = {
        champs: ['Annie', 'Lux', 'Sivir', 'Rengar', 'Leona']
    };
    let oppTeam = {
        champs: ['Zed', "Yasuo", "Alistar", "Malphite", "Orianna"]
    };
    return (
        <div className='teams-container'>
            <Team team={"blue-team"} teamdata={sameTeam} label="Blue Team" />
            <div className="vs-container">vs.</div>
            <Team team={"red-team"} teamdata={oppTeam} label="Red Team" />
        </div>
    )
}