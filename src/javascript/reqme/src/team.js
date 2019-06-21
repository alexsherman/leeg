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
    const champs = props.teamdata.champs;
    const label = props.label;
    const summonerSquares = champs.map((champ) => 
        <TeamPick key={champ} champion={champ} />
    );
    return (
        <div className={"team-container " + props.team}>
            <TeamLabel label={label} />
            {summonerSquares}
        </div>
    );
}