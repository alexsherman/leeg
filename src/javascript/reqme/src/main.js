import React, { Component } from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route, Link } from "react-router-dom";
import ChampionMatrix from "./matrix";
import DraftView from "./draft";

function App() {
    return (
        <Router>
            <div>
                <Route exact path="/" component={DraftView} />
                <Route path="/matrix" component={ChampionMatrix} />
            </div>
        </Router>
    )
}

export default App;
