import React, { Component } from "react";
import ReactDOM from "react-dom";
import {TeamPick} from './champions-squares.js';

function TeamLabel(props) {
    return (
        <div className="team-label">
            {props.label}
        </div>
    )
}

export default function Team(props) {
    const champs = props.team;
    const label = props.label;
    if (!champs) {
        return null;
    }
    const summonerSquares = champs.map((champ) => 
        <TeamPick key={champ} champion={champ} removeFn={props.removeFn} />
    );
    let unpickedSquares = [];
    for (let i = 0; i < (5 - champs.length); i++) {
        unpickedSquares.push(<TeamPick key={'unknown'+ i}/>);
    }
    return (
        <div className={"team-container " + props.teamColor}>
            <TeamLabel label={label} />
            {summonerSquares}
            {unpickedSquares}
        </div>
    );
}