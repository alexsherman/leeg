import React, { Component } from "react";
import ReactDOM from "react-dom";
import {ChampSquare} from "./main.js";

function FullScreenToggle(props) {
    console.log(props)
    return (
        <div className="matrix-cell expansion-toggle" onClick={props.expand}>
            <a>{!props.expanded ? "< >" : "> <"}</a>
        </div>
    )
}

class ChampionMatrix extends React.Component {
    constructor() {
        super();
        this.state = {
            matrix: [],
            champions: [],
            expanded: false
        }
    }

    componentDidMount() {
        this.getMatrix();
    }

    expand() {
        let matrix = document.querySelector('.matrix-container');
        if (!this.state.expanded) {
            matrix.classList.add('expanded');
        } else {
            matrix.classList.remove('expanded');
        }
        this.setState({
            expanded: !this.state.expanded
        });
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
                    matrix: j.map((champion, idx) => {
                        return {
                            name: champion.champ_name,
                            winrates_into: champion.req_service.score_vectors.same_winrates,
                            pickrates_into: champion.req_service.score_vectors.same_pickrates,
                            idx: idx
                        }
                    })
                });
                this.setState({champions: j[0].req_service.champions.list});
            }); 
        }).catch(err => {
            console.log(err);
        });
    }

    render() {
        this.state.matrix = this.state.matrix.sort((a,b) => {
            return a.name > b.name;
        });
        let vs_table = this.state.matrix.map((champion) => {
            let champ_winrates = this.state.matrix.map((vs_champ) => {
                let wr = parseFloat(vs_champ.winrates_into[champion.idx]) - 0.5;
                let style = {"backgroundColor": 'rgb(' + ((0.5 - wr * 2) * 256) + ',' + ((0.5 + wr * 2) * 256) + ',0)'};
                if (vs_champ.winrates_into[champion.idx] > 0.999) {
                    style = {"backgroundColor": "#FFFFFF", "color": "#FFFFFF"};
                }
                return (
                    <div style={style} className="matrix-cell">
                        {(vs_champ.winrates_into[champion.idx] * 100).toFixed(2) + '%'}
                    </div>
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
        let vs_header = (this.state.matrix.map((champ) => champ.name)).map(name => {
            return (
                <div className="matrix-cell matrix-header">
                    <ChampSquare champion={name} />
                    
                </div>
            )
        });
        return (
                <div className="matrix-container">
                    <FullScreenToggle expanded={this.state.expanded} expand={this.expand.bind(this)}/>
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