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
						"Ahri", "Ezreal", undefined, undefined, undefined
					]
				},
			oppTeam: {
					champs: [
						"Riven", "Malphite", undefined, undefined, undefined
					]
			}
		}
	}

	componentDidMount() {
			// open websocket, ping it if possible
	}

	componentWillUnmount() {
		// close websocket
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
					<Team teamdata={this.state.oppTeam} />
				</div>
			)
		}
}

ReactDOM.render(
  <ChampionSelect />,
  document.getElementById('app')
);

