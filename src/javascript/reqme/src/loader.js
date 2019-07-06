import React, { Component } from "react";
import ReactDOM from "react-dom";

export default function Loader() {
    return (
        <div className='loader'>
            <div className="lds-ring"><div></div><div></div><div></div><div></div></div>
        </div>
    )
}