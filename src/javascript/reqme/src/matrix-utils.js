import React, { Component } from "react";
import ReactDOM from "react-dom";
import {ChampSquare} from "./draft.js";

export function getMatrix() {
    return fetch( 'http://localhost:8000/championmatrix', {
        method: "GET",
        mode: "cors",
        headers: {
            "Accept": "application/json"
        },
        cache: 'force-cache'
    }).then(resp => {
       return resp.json().then(j => {
            return j.map((champion, idx) => {
                return {
                    name: champion.champ_name,
                    winrates_into: champion.req_service.score_vectors.same_winrates,
                    pickrates_into: champion.req_service.score_vectors.same_pickrates,
                    pickrates_against: champion.req_service.score_vectors.opp_pickrates,
                    banrates_into: champion.req_service.score_vectors.same_banrates,
                    banrates_as: champion.req_service.score_vectors.opp_banrates,
                    idx: idx
                }
            });
        }); 
    });
}

/*
* Given matrix output from getMatrix, returns the corresponding JSX for the matrix
*/
export function renderMatrixBody(stateMatrix) {
    if (!stateMatrix) {
        return;
    }
    stateMatrix = stateMatrix.sort((a,b) => {
        return a.name > b.name;
    });

    let vs_table = stateMatrix.map((champion) => {
        let champ_winrates = stateMatrix.map((vs_champ) => {
            let pr = champion.pickrates_against[vs_champ.idx];
            let wr = parseFloat(vs_champ.winrates_into[champion.idx]) - 0.5;
            let style = {"backgroundColor": 'rgb(' + ((0.5 - wr * 2) * 256) + ',' + ((0.5 + wr * 2) * 256) + ',0)'};
            if (vs_champ.winrates_into[champion.idx] > 0.999) {
                style = {"backgroundColor": "#FFFFFF", "color": "#FFFFFF"};
            }
            return (
                <MatrixCell style={style}
                 winrate={vs_champ.winrates_into[champion.idx]}
                 pickrate={pr}
                 banrate_as={vs_champ.banrates_as[champion.idx]}
                 banrate_against={vs_champ.banrates_into[champion.idx]}
                 champ_as={champion}
                 champ_against={vs_champ} />
                
            )
        });
        return (
            <React.Fragment>
                <div className="matrix-cell matrix-left-column">
                    <ChampSquare champion={champion.name} />
                </div>
                {champ_winrates}
            </React.Fragment>
        )
    });

    let vs_header = (stateMatrix.map((champ) => champ.name)).map(name => {
        return (
            <div className="matrix-cell matrix-header">
                <ChampSquare champion={name} />
                
            </div>
        )
    });

    return (
        <React.Fragment>
        {vs_header}
        {vs_table}
        </React.Fragment>
    )
}


class MatrixCell extends React.Component {
    constructor() {
        super()
        this.state = {
            expanded: false
        }
    }

    onClick() {
        console.log(this.props.banrate_as, this.props.banrate_against);
        this.setState({expanded: !this.state.expanded});
    }

    winrateInfo() {
        return this.props.champ_as.name + " wins against " + 
               this.props.champ_against.name + " " + (this.props.winrate * 100).toFixed(2) + 
               "% of the time.";
    }

    pickrateInfo() {
        return this.props.champ_against.name + " is picked against " + 
               this.props.champ_as.name + " " + (this.props.pickrate * 100).toFixed(2) + 
               "% of the time.";
    }

    banrateInfo() {
         return this.props.champ_as.name + "'s team bans " + 
                this.props.champ_against.name +  " " + (this.props.banrate_against * 100).toFixed(2) + 
                "% of the time.";
    }

    render() {
        return (
            <div style={this.props.style} 
                 className={this.state.expanded ? "matrix-cell expanded" : "matrix-cell"}
                 onClick={this.onClick.bind(this)}
            >
                {this.state.expanded 
                    ?
                    <React.Fragment> 
                    <p>{this.winrateInfo()}</p>
                    <p>{this.pickrateInfo()}</p>
                    <p>{this.banrateInfo()}</p>
                    </React.Fragment>
                    : 
                    (this.props.winrate * 100).toFixed(2) + "%"
                }
            </div>)
        }
}
