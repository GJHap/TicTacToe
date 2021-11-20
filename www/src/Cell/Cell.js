import React from 'react';
import './Cell.css'


export default class Cell extends React.Component {
    render() {
        return <button className="cell" onClick={this.props.onClick}>{this.props.value}</button>
    }
}