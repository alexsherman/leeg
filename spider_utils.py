import requests
from requests.exceptions import HTTPError
import time
import json
import os
import sys 
_url_root = 'https://na1.api.riotgames.com'
_summoner_path = '/lol/summoner/v4/summoners/by-name/'
_match_history_path = '/lol/match/v4/matchlists/by-account/'
_match_info_path = '/lol/match/v4/matches/'

try:
    key = os.environ['RIOT_API_KEY']
except KeyError:
    print('You forgot to export your development key as RIOT_API_KEY!')
    sys.exit(1)

def makeRequest(url, optional_params = {}):
    '''
    Make an http request to the given URL with the specified parameters.
    
    Args:
        url: String of the full url
        optional_params: Dict to be sent as query params
    
    Returns:
        requests response object
    
    Raises:
        HTTPError: raises exception for httperror
    '''
    # dev key allows 100 requests per 2 min - sleep before each request to be sure we don't exceed
    rate_limit = 120/100
    try: 
        optional_params.update({'api_key': key})
        time.sleep(rate_limit)
        response = requests.get(url, params = optional_params)
        response.raise_for_status()
        return response
    except HTTPError as http_err:
        print(http_err)
        raise

def getSummonerByName(name):
    '''
    Given name, looks up a given summoner
    Args:
        name: string, summoner name
    Returns:
        SummonerDTO object as json
    '''
    summoner_request_url = _url_root + _summoner_path + name
    summoner_response = makeRequest(summoner_request_url)
    return summoner_response.json()

# default - get last 10 matches of summoner. caller can specify 
# see https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchlist
def getSummonerMatchHistory(encrypted_account_id, params = {'endIndex': 10}):
    '''
    Given an encrypted account id, returns a MatchlistDto for the given params
    Args:
        encrypted_account_id: account id of a summoner. 
            Either SummonerDTO['accountId'] or PlayerDto['currentAccountId']
        params: option dict of query paramters to specify which matches to retrieve.
            See https://developer.riotgames.com/api-methods/#match-v4/GET_getMatch 
    Returns:
        MatchlistDto as json
    '''
    match_history_request_url = _url_root + _match_history_path + encrypted_account_id
    return makeRequest(match_history_request_url, params).json()

def getMatchIds(match_history):i
    ''' Given MatchlistDto, return list of match ids.'''
    return map(lambda m: m['gameId'], match_history['matches'])

def getMatch(match_id):
    ''' Given a match id, returns MatchDto as json.'''
    match_request_url = _url_root + _match_info_path + str(match_id)
    return makeRequest(match_request_url).json()

