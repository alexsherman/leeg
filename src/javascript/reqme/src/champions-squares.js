import React, { Component } from "react";
import ReactDOM from "react-dom";

export function SummonerSquare(props) {
    const champ = props.championName;
    return (
        <div className="summoner-square">
            <ChampSquare champion={champ} />
            <div className="champion-name">{champ}</div>
        </div>
    )
}

export function TeamPick(props) {
    const champName = props.champion || 'None';
    const champStats = {
        winrate: 51.23,
        pickrate: 4.23,
        banrate: 6.09
    };
    return (
        <div className="team-pick">
            <div className='champ-button-group'>
                <div 
                    className='champ-button add-to-bans'
                    onClick={(e) => props.removeFn({name: props.champion})}
                >
                </div>
            </div>
            <SummonerSquare championName={champName} /> 
            {props.champion && <ChampionStats champStats={champStats} />}
        </div>
    )
}

function ChampionStats(props) {
    return (
        <div className="team-pick-stats">
            <span className="winrate">{props.champStats.winrate}% wr</span><br />
            <span className="pickrate">{props.champStats.pickrate}% pr</span><br />
            <span className="banrate">{props.champStats.banrate}% br</span><br />
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