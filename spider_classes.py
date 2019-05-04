import json

champ_dict = {}

with open('champions.json') as champs:
    try:
        champ_dict = json.load(champs)
    except:
        print("No champions.json found!")
        raise

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

    def writeNamesAndLanes(self, writer):
        blue = []
        red = []
        for pc in self.playersAndChamps:
            info = {
                'champion': pc['champion']['name'],
                'role': pc['role']
            }
            if pc['team'] is 'blue':
                blue.append(info)
            else:
                red.append(info)
        writer.writerow([
            self.id,
            self.winner,
            blue,
            red,
            self.game_version
        ])


class PlayerChamp: 
    def __init__(self, player, participants):
        self.participantId = player['participantId']
        self.champion = Champ(player['championId'])
        self.team = 'blue' if player['teamId'] == 100 else 'red'
        participant = next((p for p in participants if p['participantId'] == self.participantId), None)
        if participant is not None:
            self.id = participant['player']['currentAccountId']
            self.name = participant['player']['summonerName']
       # self.rank = player['highestAchievedSeasonTier']
        self.role = player['timeline']['lane']
        if self.role in ('BOT', 'BOTTOM'):
           self.role = player['timeline']['role']

    def info(self):
        return {
            'champion': self.champion.info(),
            'role': self.role,
            'team': self.team,
            'summonerName': self.name,
            'accountId': self.id
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
