import requests
import os
import time
import json
import csv
import argparse

url_root = 'https://na1.api.riotgames.com'
summoner_path = '/lol/summoner/v4/summoners/by-name/'
match_history_path = '/lol/match/v4/matchlists/by-account/'
match_info_path = '/lol/match/v4/matches/'
key = os.environ['RIOT_API_KEY']
champ_dict = {}

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
            print(participant)
            self.id = participant['player']['summonerId']
       # self.rank = player['highestAchievedSeasonTier']
        self.role = player['timeline']['lane']
        if self.role in ('BOT', 'BOTTOM'):
           self.role = player['timeline']['role']

    def info(self):
        return {
            'champion': self.champion,
            'team': self.team,
            'summonerId': self.id
        }

 
def initCSV():
    with open('matches.csv', mode='w') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        match_writer.writerow(['match_id', 'winning_team', 'team1champs', 'team2champs', 'team1bans', 'team2bans', 'game_version'])


class Champ:
    def __init__(self, champ_id):
        self.id = champ_id
        if champ_id == -1:
            self.name = 'None'
        else:
            self.name = champ_dict[str(champ_id)]
    def __str__(self):
        return "% - %" & (self.name, self.id)

def getChampById(champ_id):
    if champ_id == -1:
        return 'None'
    else:
        return champ_dict[str(champ_id)]


def recordMatch(match):
    with open('matches.csv', mode='w') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        m = Match(match)
        m.write(match_writer)
         
'''
def recordMatch(match):
    with open('matches.csv', mode='a') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        winner = ""
        team1 = match['teams'][0]
        team2 = match['teams'][1]
        if team1['win'] == 'Win':
            winner = team1['teamId']
        else:
            winner = team2['teamId']
        print(map(lambda b: b['championId'], team2['bans']))
        team1bans = map(getChampById, map(lambda b: b['championId'], team1['bans']))
        team2bans = map(getChampById, map(lambda b: b['championId'], team2['bans']))

        champsByTeam = {
            '100': [],
            '200': []
        }
        for champ in match['participants']:
            champsByTeam[str(champ['teamId'])].append(champ_dict[str(champ['championId'])])
        match_writer.writerow([match['gameId'], winner, champsByTeam['100'], champsByTeam['200'], team1bans, team2bans, match['gameVersion']])
'''
    
def makeRequest(url):
    # dev key allows 100 requests per 2 min - sleep before each request to be sure we don't exceed
    rate_limit = 120/100
    time.sleep(rate_limit)
    return requests.get(url, params={'api_key': key})

def getSummonerByName(name):
    summoner_request_url = url_root + summoner_path + name
    summoner_response = makeRequest(summoner_request_url)
    return summoner_response.json()

def getSummonerMatchHistory(summonerInfo):
    encrypted_account_id = summonerInfo['accountId']
    match_history_request_url = url_root + match_history_path + encrypted_account_id
    return makeRequest(match_history_request_url).json()

def getMatchIds(match_history):
    return map(lambda m: m['gameId'], match_history['matches'])

def getMatch(match_id):
    match_request_url = url_root + match_info_path + str(match_id)
    return makeRequest(match_request_url).json()

def genChampDict():
    champs = requests.get('http://ddragon.leagueoflegends.com/cdn/6.24.1/data/en_US/champion.json').json()
    champ_dict = {}
    for champ in champs["data"].values():
        champ_dict[champ["key"]] = champ["name"]
    file = open('champions.json', 'w')
    file.write(json.dumps(champ_dict))
    file.close()
    return champ_dict

if __name__ == '__main__':
    parser = argparse.ArgumentParser("Scrape Riot API data")
    parser.add_argument('--summoner_name', help="name of summoner, seed for spider", default="sleepo mode")
    parser.add_argument('--n_matches', help="number of matches to crawl through", default=1000, type=int)
    args = parser.parse_args()
    with open('champions.json') as json_file:
        champ_dict = json.load(json_file)
    #initCSV()
    seed_summoner = getSummonerByName(args.summoner_name)
    seed_summoner_match_history = getSummonerMatchHistory(seed_summoner)
    match_ids = getMatchIds(seed_summoner_match_history);
    for match_id in match_ids:
        match = getMatch(match_id)
        champs = map(lambda p: p['championId'], match['participants'])
        print(champs)
        print(map(lambda c: champ_dict[str(c)], champs))
        recordMatch(match)
    
