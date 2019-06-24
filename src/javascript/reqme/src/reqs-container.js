import React, { Component } from "react";
import ReactDOM from "react-dom";
import Select from 'react-select';
import { SummonerSquare, ChampSquare } from './champions-squares.js';

export function ReqsContainer(props) {
    return (
        <div className='reqs-container'>
           <ReqsButton getReqs={props.reqHelpers.getReqs} />
           <Reqs reqs={props.reqHelpers.reqs}
                 roles={props.reqHelpers.roles} 
                 updateRoles={props.reqHelpers.updateRoles} 
                 allChampions={props.allChampions}
            />
        </div>
    )
}

function ReqsButton(props) {
     return (
        <div className='get-reqs-button'
            onClick={(e) => props.getReqs()}>
            Click to get reqs for these teams
        </div>
     )  
}

function RoleToggles(props) {
    const options = [
        {"value": "Top", "label": "Top"},
        {"value": "Jungle", "label": "Jungle"},
        {"value": "Mid", "label": "Mid"},
        {"value": "Bottom", "label": "Bottom"},
        {"value": "Support", "label": "Support"},
    ];
    const placeholder = "Select one or more roles to filter recommendations."
    return (
        <Select
        className="role-select"
        isMulti
        placeholder={placeholder}
        onChange={props.updateRoles}
        options={options}
      />
    )
}


function Reqs(props) {
    if (props.reqs === null || props.reqs === undefined) {
        return null;
    }
    const reqs = props.reqs;
    reqs.splice(5);
    const indivReqs = reqs.map((champName, idx) => {
        /*let champ = props.allChampions.find(champion => {
            return champion.name === champName;
        });*/
        return (<React.Fragment>
            <SummonerSquare key={champName} championName={champName} />
        </React.Fragment>)
    });
    return (
        <div className="center-container">
            <RoleToggles roles={props.roles} updateRoles={props.updateRoles} />
            <div className="req-container">
                {indivReqs}            
            </div>
        </div>
    );
}