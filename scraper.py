import requests
import os
import time

url_root = 'https://na1.api.riotgames.com'
summoner_path = '/lol/summoner/v4/summoners/by-name/'
match_history_path = '/lol/match/v4/matchlists/by-account/'
match_info_path = '/lol/match/v4/matches/'
key = os.environ['RIOT_API_KEY']

def makeRequest(url):
    return requests.get(url, params={'api_key': key})

def getSummonerByName(name):
    summoner_request_url = url_root + summoner_path + name
    summoner_response = makeRequest(summoner_request_url)
    return summoner_response.json()

def getSummonerMatchHistory(summonerInfo):
    encrypted_account_id = summonerInfo['accountId']
    match_history_request_url = url_root + match_history_path + encrypted_account_id
    return makeRequest(match_history_request_url).json()

def getMatch():
    print('getMatch')

if __name__ == '__main__':
    sleepo_mode = getSummonerByName('sleepo mode')
    sleepo_match_history = getSummonerMatchHistory(sleepo_mode)
    print(sleepo_match_history)
    
