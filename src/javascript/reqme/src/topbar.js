import React, { Component } from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route, Link } from "react-router-dom";


export default function Sidebar(props) {
    return (
        <div className='topbar-container'>
            <div className='topbar-inner-container'>
                <TopbarItem name={'smartdraft'}/>
                <TopbarItem name={'tierlist'}/>
                <TopbarItem name={'championtensor'}/>
            </div>        
        </div>
    )
}

function TopbarItem(props) {
    return (
        <div className='topbar-item'>
            {props.name}
        </div>
    )
}

function Logo(props) {
    return (
        <div className="logo">
            req.gg
            <div className="subtitle">
                 reqs of extreme quality
            </div>
        </div>
    );
}
