import React, { Component } from "react";
import ReactDOM from "react-dom";
import {getMatrix, renderMatrixBody, MatrixCell} from "./matrix-utils";

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
            expanded: false
        }
    }

    componentDidMount() {
        this.setMatrix();
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

    setMatrix() {
        getMatrix().then(matrix => {
            console.log(matrix);
            this.setState({
                matrix: matrix
            });
        });
    }

    render() {
       let matrixBody = renderMatrixBody(this.state.matrix);
        return (
            <div className="matrix-container">
                <FullScreenToggle expanded={this.state.expanded} expand={this.expand.bind(this)}/>
                {matrixBody}
            </div>
        )   
   }
}

export default ChampionMatrix;