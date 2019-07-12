import requests
from requests.exceptions import HTTPError
import datetime
import time
import json
import os
import sys 
from random import shuffle
_url_root = 'https://na1.api.riotgames.com'
_summoner_path = '/lol/summoner/v4/summoners/by-name/'
_summoner_by_id_path = '/lol/summoner/v4/summoners/'
_match_history_path = '/lol/match/v4/matchlists/by-account/'
_match_info_path = '/lol/match/v4/matches/'
_mastery_path = '/lol/champion-mastery/v4/champion-masteries/by-summoner/'
_ranked_league_path = '/lol/league/v4/entries/RANKED_SOLO_5x5/'
LEAGUE_DIVISIONS = ["I", "II", "III", "IV"]
LEAGUE_TIERS = ["IRON", "BRONZE", "SILVER", "GOLD", "PLATINUM", "DIAMOND"] #todo other tiers

try:
    key = os.environ['RIOT_API_KEY']
except KeyError:
    print('You forgot to export your development key as RIOT_API_KEY!')
    sys.exit(1)

def makeRequest(url, optional_params = {}, print_err = True):
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
        response = requests.get(url, params = optional_params)
        response.raise_for_status()
        time.sleep(rate_limit)
        return response
    except HTTPError as http_err:
        if print_err:
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
    return makeRequest(match_history_request_url, params, print_err = False).json()

def getMatchIds(match_history):
    ''' Given MatchlistDto, return list of match ids.'''
    return map(lambda m: m['gameId'], match_history['matches'])

def getMatch(match_id):
    ''' Given a match id, returns MatchDto as json.'''
    match_request_url = _url_root + _match_info_path + str(match_id)
    return makeRequest(match_request_url).json()

def getSummonerMasteries(encrypted_summoner_id):
    mastery_request_url = _url_root + _mastery_path + encrypted_summoner_id
    return makeRequest(mastery_request_url).json()


def getLeagueEntriesForDivision(tier, division, page):
    league_url = _url_root + _ranked_league_path + tier + '/' + division
    params = {
        'page': page
    }
    return makeRequest(league_url, params).json()

def getLeagueEntriesForTier(tier, page): 
    if tier in ('MASTER', 'GRANDMASTER', 'CHALLENGER'):
        #todo - handle later because these require different routes without divisions
        return
    results = []
    for division in LEAGUE_DIVISIONS:
        try:
            print('Getting entries for {} {}'.format(tier, division))
            league_entries = getLeagueEntriesForDivision(tier, division, page)
            results = results + league_entries;
        except Exception as e:
            raise e
    return results


def getAccountIdFromSummonerId(summoner_id):
    url = _url_root + _summoner_by_id_path + summoner_id
    return makeRequest(url).json()['accountId']

def getMatchesFromLeagueEntries(leagueEntries):
    matches = [];
    n = 0
    n_days = 7
    for entry in leagueEntries:
        account_id = getAccountIdFromSummonerId(entry['summonerId'])
        beginTime = int(time.time()- n_days * 24 * 60 * 60) * 1000;
        params = {
            'beginTime': beginTime,
            'queue': [420]
        }
        try: 
            match_history = getSummonerMatchHistory(account_id, params)
            n_matches = len(match_history['matches'])
            print("Got {} matches from {} ".format(n_matches, account_id))
            n += 1
            if n > 50:
                break;
            if n_matches > 0:
                matches = matches + match_history['matches']
        except Exception as e:
            print("No matches found for {} in last {} days".format(account_id, n_days))
    return matches

def all_matches_today(page):
    while True:
        matches = []
        for tier in LEAGUE_TIERS:
            try:
                entries = getLeagueEntriesForTier(tier, page)
                shuffle(entries)
                tier_matches = getMatchesFromLeagueEntries(entries)
                for match in tier_matches:
                    match['approximateTier'] = tier
                matches = matches + tier_matches
            except Exception as e:
                print("Error getting entries for {} - page {} {}".format(tier, page, e))
        page += 1
        if len(matches) == 0 and page > 10:
            break
        yield matches


        