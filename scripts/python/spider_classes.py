import json
import datetime
import psycopg2
champ_dict = {}
champ_indexes = {}

with open('../../champions.json') as champs:
    try:
        champ_dict = json.load(champs)
        idx = 0
        for name in champ_dict.values():
            champ_indexes[name] = idx
            idx += 1
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
        self.play_date = match['gameCreation']
        self.duration = match['gameDuration']
        self.game_mode = match['queueId']
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
        self.blue_wins = self.winner == 'blue'
        self.blue_team = []
        self.red_team = []
        self.blue_bans = []
        self.red_bans = []
        self.red_summoners = []
        self.blue_summoners = []
        for pc in self.playersAndChamps:
            champ = pc['champion']['name']
            summoner = pc['accountId'] 
            if pc['team'] is 'blue':
                self.blue_team.append(champ)
                self.blue_summoners.append(summoner)
            else:
                self.red_team.append(champ)
                self.red_summoners.append(summoner)
        for ban in blueTeam['bans']:
            self.blue_bans.append(getChampById(ban['championId']))
        for ban in redTeam['bans']:
            self.red_bans.append(getChampById(ban['championId']))

    def insert_into_all_matches(self, cursor):
        sql = '''
                INSERT INTO all_matches
                (id, blue_wins, blue_team, red_team, blue_bans, red_bans, 
                blue_summoners, red_summoners, play_date, duration, game_version, game_mode)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
            '''
        play_date = datetime.datetime.fromtimestamp(self.play_date / 1000)
        duration = datetime.timedelta(seconds=self.duration)
        sql_tuple = (self.id, self.blue_wins, self.blue_team, self.red_team, self.blue_bans, self.red_bans, self.blue_summoners, self.red_summoners, play_date, duration, self.game_version, self.game_mode)
        try:
            cursor.execute(sql, sql_tuple)
            print('Writing {} to all_matches'.format(self.id))
        except psycopg2.DatabaseError as e:
            print(e)
    
    def insert_into_summoner_matches(self, cursor):
        '''
        TODO implement
        '''
        return
        
    def writeVectors(self, writer):
        blue = [0] * len(champ_indexes)
        red = [0] * len (champ_indexes)
        for pc in self.playersAndChamps:
            if pc['team'] is 'blue':
                blue[champ_indexes[pc['champion']['name']]] = 1
            else:
                red[champ_indexes[pc['champion']['name']]] = 1        
        writer.writerow([
             self.id,
             self.winner,
            *blue,
            *red,
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
