import React, { Component } from "react";
import ReactDOM from "react-dom";
import {ChampSquare} from "./main.js";

class ChampionMatrix extends React.Component {
    constructor() {
        super();
        this.state = {
            matrix: [],
            champions: []
        }
    }

    componentDidMount() {
        this.getMatrix();
    }

    getMatrix() {
        fetch( 'http://localhost:8000/championmatrix', {
            method: "GET",
            mode: "cors",
            headers: {
                "Accept": "application/json"
            }
        }).then(resp => {

            resp.json().then(j => {
                this.setState({
                    matrix: j
                });
                this.setState({champions: j[0].req_service.champions.list});
            }); 
        }).catch(err => {
            console.log(err);
        });
    }

    render() {
        let vs_champions = this.state.matrix.map((champion) => {
            return {
                name: champion.champ_name,
                winrates_into: champion.req_service.score_vectors.opp_winrates.map((score) => {
                    if (score === 1) {
                        return "x";
                    }
                    return score.toFixed(2);
                }),
                pickrates_into: champion.req_service.score_vectors.same_pickrates
            }
        });
        console.log(vs_champions);
        let vs_table = this.state.champions.map((champion, idx) => {
            let champ_winrates = vs_champions.map((vs_champ) => {
                let wr = parseFloat(vs_champ.winrates_into[idx]) - 0.5;
                let style = {"background-color": 'rgb(' + ((0.5 - wr * 2) * 256) + ',' + ((0.5 + wr * 2) * 256) + ',0)'};

                return (
                    <div style={style} className="matrix-cell">
                        {vs_champ.winrates_into[idx]}
                    </div>
                )
            });
            return (
                <React.Fragment>
                    <div className="matrix-cell matrix-left-column">
                        <ChampSquare champion={champion.name} />
                        {champion.name}
                    </div>
                    {champ_winrates}
                </React.Fragment>
            )
        });
        console.log(vs_table);
        let vs_header = ["",].concat(vs_champions.map((champ) => champ.name)).map(name => {
            return (
                <div className="matrix-cell matrix-header">
                    <ChampSquare champion={name} />
                    {name}
                </div>
            )
        });
        return (
                <div className="matrix-container">
                    {vs_header}
                    {vs_table}
                </div>
            )
        }
}


ReactDOM.render(
  <ChampionMatrix />,
  document.getElementById('app')
);

export default ChampionMatrix;