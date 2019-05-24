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
let nws = undefined;

wss.on('connection', function open(ws) {
    nws = ws;
    console.log('connected');
    ws.send(JSON.stringify(teams));
});

let c = 0;
let team = teams.sameTeam;
rl.on('line', (input) => {
    if (input === 'clear') {
        console.log('clearing teams');
        c = 0;
        team = teams.sameTeam;
        teams = {
            sameTeam: {
                champs: []
            },
            oppTeam: {
                champs: []
            }
        }
    } else {
        console.log('Adding ', input);
        team.champs.push(input);
        console.log(teams);     
        c += 1;
        team = c % 2 === 0 ? teams.sameTeam : teams.oppTeam;
    }
    nws.send(JSON.stringify(teams));
}).on('close', () => {
    console.log('test ending');
    process.exit(0);
});