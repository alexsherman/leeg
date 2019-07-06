import React, { Component } from "react";
import ReactDOM from "react-dom";
import { SummonerSquare, ChampSquare } from './champions-squares.js';

function ChampButtonGroup(props) {
    function removeFromAllTeams(champion) {
        props.teamFns.removeChampFromSameTeam(champion);
        props.teamFns.removeChampFromOppTeam(champion);
    }
    return (
        <div className='champ-button-group'>
            <div 
                className='champ-button add-to-blue'
                onClick={(e) => props.teamFns.addChampToSameTeam(props.champion)}
            >  
            </div>
            <div 
                className='champ-button add-to-red'
                onClick={(e) => props.teamFns.addChampToOppTeam(props.champion)}
            >
            </div>
            <div 
                className='champ-button add-to-bans'
                onClick={(e) => removeFromAllTeams(props.champion)}
            >
            </div>
        </div>
    )

}

export default function ChampionPicker(props) {
    if (!props.champions.length) {
       return (
            <div className="champ-list">
            </div>
        )
    }
    const champs = props.champions.map(champ => {
        return <div className="champ-and-options">
            <ChampSquare champion={champ.name} />
            <ChampButtonGroup 
                teamFns={props.teamFns} 
                champion={champ}
            />
        </div>
    });

    return (
        <div className="champ-list">
            {champs}
        </div>
    )
}