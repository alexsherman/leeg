import json
import datetime
import psycopg2

def getTeam(match, teamColor):
    idsToColors = {
        100: 'blue',
        200: 'red'
    }
    return next((t for t in match['teams'] if idsToColors[t['teamId']] == teamColor), None)

class Match: 
    def __init__(self, match, tier = 'None'):
        self.id = match['gameId']
        self.play_date = match['gameCreation']
        self.duration = match['gameDuration']
        self.game_mode = match['queueId']
        self.tier = tier
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
        self.red_roles = []
        self.blue_roles = []
        
        for pc in self.playersAndChamps:
            champid = pc['champion']['id']
            role = pc['role'] 
            if pc['team'] is 'blue':
                self.blue_team.append(champid)
                self.blue_roles.append(role)
            else:
                self.red_team.append(champid)
                self.red_roles.append(role)
        
        for ban in blueTeam['bans']:
            self.blue_bans.append(ban['championId'])
        
        for ban in redTeam['bans']:
            self.red_bans.append(ban['championId'])

    def insert_into_all_matches(self, cursor):
        sql = '''
                INSERT INTO all_matches_2
                (id, blue_wins, blue_team, red_team, blue_bans, red_bans, 
                blue_roles, red_roles, play_date, game_version, rank)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
            '''
        play_date = datetime.datetime.fromtimestamp(self.play_date / 1000)
        game_version_list = self.game_version.split('.', 2)
        game_version = game_version_list[0] + '.' + game_version_list[1]
        sql_tuple = (self.id, self.blue_wins, self.blue_team, self.red_team, self.blue_bans, self.red_bans, 
                    self.blue_roles, self.red_roles, play_date, game_version, self.tier)
        try:
            cursor.execute(sql, sql_tuple)
            print('Writing {} to all_matches'.format(self.id))
        except psycopg2.DatabaseError as e:
            print(e)
    
    def insert_into_summoner_matches(self, cursor, summoner):
        same_team_champions = self.blue_team
        opp_team_champions = self.red_team
        same_team_bans = self.blue_bans
        opp_team_bans = self.red_bans
        
        if summoner.team == 'red':
            opp_team_champions = self.blue_team
            same_team_champions = self.red_team
            same_team_bans = self.red_bans
            opp_team_bans = self.blue_bans
        
        same_team_champions.remove(summoner.champion)
        play_date = datetime.datetime.fromtimestamp(self.play_date / 1000)
        duration = datetime.timedelta(seconds=self.duration)
        sql = '''
                INSERT INTO summoner_matches
                (id, name, wins, champion, same_team_champions, opp_team_champions, same_team_bans,
                opp_team_bans, match_id, play_date, game_version)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
            '''
        sql_tuple = (summoner.id, summoner.name, summoner.wins, summoner.champion, same_team_champions, opp_team_champions, same_team_bans, opp_team_bans, self.id, play_date, self.game_version)
        try:
            cursor.execute(sql, sql_tuple)
            print('Writing {} to summoner_matches for {}'.format(self.id, summoner.name))
        except psycopg2.DatabaseError as e:
            print(e)

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

    
class Summoner:
    def from_match(self, m, id): 
        pc = next((p for p in m.playersAndChamps if p['accountId'] == id), None) 
        self.id = id
        self.team = pc['team']
        self.name = pc['summonerName']
        self.champion = pc['champion']['name']
        self.wins = m.winner == pc['team']
        return self

class PlayerChamp: 
    def __init__(self, player, participants):
        self.participantId = player['participantId']
        self.champion = Champ(player['championId'])
        self.team = 'blue' if player['teamId'] == 100 else 'red'
        participant = next((p for p in participants if p['participantId'] == self.participantId), None)
        if participant is not None:
            self.id = participant['player']['currentAccountId']
            self.name = participant['player']['summonerName']
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
    def __str__(self):
        return "% - %" & (self.name, self.id)
    def info(self):
        return {
            'id': self.id
        }
