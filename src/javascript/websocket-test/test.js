const WebSocket = require('ws');
const readline = require('readline');
const wss = new WebSocket.Server({ port: 5000});
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

let teams = {
    sameTeam: {
        champs: []
    },
    oppTeam: {
        champs: []
    },
}

wss.on('connection', function open(ws) {
    console.log('connected');
    ws.send(JSON.stringify(teams));
    let c = 0;
    let team = teams.sameTeam;
    rl.on('line', (input) => {
        console.log('Adding ', input);
        team.champs.push(input);
        console.log(teams);
        ws.send(JSON.stringify(teams));
        c += 1;
        team = c % 2 === 0 ? teams.sameTeam : teams.oppTeam;
    }).on('close', () => {
        console.log('test ending');
        process.exit(0);
    });
});