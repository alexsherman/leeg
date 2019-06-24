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

export function TeamsContainer(props) {
    return (
        <div className='teams-container'>
            <Team teamColor={"blue-team"} team={props.sameTeam} label="Blue Team" />
            <div className="vs-container">vs.</div>
            <Team teamColor={"red-team"} team={props.oppTeam} label="Red Team" />
        </div>
    )
}