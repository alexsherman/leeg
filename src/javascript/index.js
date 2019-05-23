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
			req: null
		}
	}

	componentDidMount() {
			this.getReqs();
			// open websocket, ping it if possible
	}

	componentWillUnmount() {
		// close websocket
	}

	getReqs() {
		const baseUrl = 'http://localhost:8000/globalreq';
		const params = '?' + 'team=' + this.state.sameTeam.champs.join(',') + '&opp=' + this.state.oppTeam.champs.join(',');
		fetch(baseUrl + params, {mode: 'no-cors'}).then(resp => {

			resp.body.then(text => {
				console.log(text)
				this.setState({
					req: text
				});
			});	
		});
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
					<h1>We recommend {this.state.req}</h1>
					<Team teamdata={this.state.oppTeam} />
				</div>
			)
		}
}

ReactDOM.render(
  <ChampionSelect />,
  document.getElementById('app')
);

