import React, { Component } from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route, Link } from "react-router-dom";


export default function Sidebar(props) {
    return (
        <div className='sidebar-container'>
            <Logo />
            <Link to="/">Draft</Link>
            <Link to="/matrix">Matrix</Link>
            <NotEndorsedNotice />
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

function NotEndorsedNotice(props) {
    return (
        <div className="notice">
             req.gg isn't endorsed by Riot Games and doesn't reflect 
             the views or opinions of Riot Games or anyone officially 
             involved in producing or managing League of Legends. 
             League of Legends and Riot Games are trademarks or registered 
             trademarks of Riot Games, Inc. League of Legends © Riot Games, Inc
        </div>
    )
}