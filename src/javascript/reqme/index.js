function SummonerSquare(props) {
    const champ = props.champion;
    return (
        <h1>{champ}</h1>
    )
}

function Team(props) {
    const champs = props.teamdata.champs;
    const summonerSquares = champs.map((champ) => 
        <SummonerSquare key={champ} champion={champ} />
    );
    return (
        <div className="team-container">
            {summonerSquares}
        </div>
    );
}

function Reqs(props) {
    if (props.resp === null) {
        return;
    }
    const reqs = props.resp;
    const indivReqs = reqs.map((champ) => 
        <li key={champ}>{champ}</li>
    );
    return (
        <div className="req-container">
            We recommend:
            <ul>
                {indivReqs}
            </ul>
        </div>
    )
}

class ChampionSelect extends React.Component {
    constructor() {
        super();
        this.state = {
            sameTeam: {
                    champs: [
                        "Ahri", "Ezreal",
                    ]
                },
            oppTeam: {
                    champs: [
                        "Riven"
                    ]
            },
            req: []
        }
    }

    componentDidMount() {
            //this.getReqs();
            // open websocket, ping it if possible
        let tm = undefined;
        let webSocket = new WebSocket('ws://localhost:5000');
        webSocket.onmessage = event => {
            const teams = JSON.parse(event.data);
            this.setState({
                sameTeam: teams.sameTeam,
                oppTeam: teams.oppTeam
            });
            if (tm) {
                tm.clearTimeout
            }
            tm = setTimeout(this.getReqs.bind(this), 500);
        }
    }

    componentWillUnmount() {
        // close websocket
    }

    getReqs() {
        const baseUrl = 'http://localhost:8000/globalreq';
        let params = '?';
        if (this.state.sameTeam.champs.length > 0) {
            params += 'team=' + this.state.sameTeam.champs.join(',');
        }
        if (this.state.oppTeam.champs.length > 0) {
            params += '&opp=' + this.state.oppTeam.champs.join(',');
        }
        if (this.state.oppTeam.champs.length === 0 && this.state.sameTeam.champs.length === 0) {
            this.setState({
                    req: []
                });
            return;
        }

        fetch(baseUrl + params, {
            method: "GET",
            mode: "cors",

            headers: {
                "Accept": "application/json"
            }
        }).then(resp => {

            resp.json().then(text => {
                console.log(text)
                this.setState({
                    req: text.reqs
                });
            }); 
        }).catch(err => {
            console.log(err);
        })
    }

    addChampToTeam() {
        this.setState(prevState => ({
                 team: {
                    champs: [...prevState.team.champs, "bew cgano"]
                 }
        }));
    }

    render() {
        return (
                <div id="app-container">
                    <Team teamdata={this.state.sameTeam} />
                    <Reqs resp={this.state.req} />
                    <Team teamdata={this.state.oppTeam} />
                </div>
            )
        }
}

ReactDOM.render(
  <ChampionSelect />,
  document.getElementById('app')
);

