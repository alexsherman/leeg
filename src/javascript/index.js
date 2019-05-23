function SummonerSquare(props) {
	const champ = props.champion;
	return (
		<h1>{champ}</h1>
	)
}

function Team(props) {
	const champs = props.teamdata.champs;
	const summonerSquares = champs.map((champ) => 
		<SummonerSquare key={champ.toString()} champion={champ} />
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
			team: {
					champs: [
						"Ahri", "Ezreal", "Riven", "Sona", "Karthus"
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
				<div>
					<Team teamdata={this.state.team} />
				</div>
			)
		}
}

ReactDOM.render(
  <ChampionSelect />,
  document.getElementById('app-container')
);

