import json

champ_dict = {}

with open('champions.json', mode='r') as champs:
	champ_dict = champs

def getTeam(match, teamColor):
    idsToColors = {
        100: 'blue',
        200: 'red'
    }
    return next((t for t in match['teams'] if idsToColors[t['teamId']] == teamColor), None)

class Match: 
    def __init__(self, match):
        self.id = match['gameId']
        blueTeam = getTeam(match, 'blue')
        redTeam = getTeam(match, 'red')
        if blueTeam['win'] == 'Win':
            self.winner = 'blue'
        else:
            self.winner = 'red'
        self.game_version = match['gameVersion']
        self.playersAndChamps = []
        for participant in match['participants']:
            p = PlayerChamp(participant, match['participantIdentities'])
            self.playersAndChamps.append(p.info())
    def write(self, writer):
        writer.writerow([
            self.id,
            self.winner,
            self.playersAndChamps,
            self.game_version
        ])

class PlayerChamp: 
    def __init__(self, player, participants):
        self.participantId = player['participantId']
        self.champion = Champ(player['championId'])
        self.team = 'blue' if player['teamId'] == 100 else 'red'
        participant = next((p for p in participants if p['participantId'] == self.participantId), None)
        if participant is not None:
            self.id = participant['player']['summonerId']
       # self.rank = player['highestAchievedSeasonTier']
        self.role = player['timeline']['lane']
        if self.role in ('BOT', 'BOTTOM'):
           self.role = player['timeline']['role']

    def info(self):
        return {
            'champion': self.champion.info(),
            'team': self.team,
            'summonerId': self.id
        }

class Champ:
    def __init__(self, champ_id):
        self.id = champ_id
        if champ_id == -1:
            self.name = 'None'
        else:
            self.name = champ_dict[str(champ_id)]
    def __str__(self):
        return "% - %" & (self.name, self.id)
    def info(self):
        return {
            'name': self.name,
            'id': self.id
        }

def getChampById(champ_id):
    if champ_id == -1:
        return 'None'
    else:
        return champ_dict[str(champ_id)]   
