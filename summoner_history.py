import os
import sys
import json
import csv
import argparse
import atexit
from math import ceil
from queue import Queue, Empty
from spider_utils import *
from spider_classes import *

_match_queue = Queue()
_5v5_queue_ids = [400, 420]

def initCSV():
    with open('summoner_matches.csv', mode='w+') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        match_writer.writerow(['summoner_name', 'summoner_champ', 'summoner_win', 'same_team_champs', 'opposite_team_champs', 'account_id', 'match_id', 'game_version'])    

def recordMatch(account_id, MatchReferenceDto, match):
    '''
    Given MatchDto, converts it to Match instance and appends it to csv.
    TODO: various csvs, allow command line arg specify file

    Args:
        match: MatchDto
    '''
    m = Match(match)
    
    with open('summoner_matches.csv', mode='a') as match_csv:
        match_writer =  csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        print('Writing match ' + str(m.id))
        summoner_in_match = next(c for c in m.playersAndChamps if c['accountId'] == account_id)
        summoner_win = summoner_in_match['team'] == m.winner
        same_team_champs = []
        opposite_team_champs = []
        for c in m.playersAndChamps:
            if c['accountId'] == account_id:
                continue
            elif c['team'] == summoner_in_match['team']:
                same_team_champs.append(c['champion']['name'])
            else:
                opposite_team_champs.append(c['champion']['name'])
        
        match_writer.writerow([
            summoner_in_match['summonerName'],
            summoner_in_match['champion']['name'],
            summoner_win,
            same_team_champs,
            opposite_team_champs,
            account_id,
            m.id,
            m.game_version
        ])

def crawl(args, account_id):
    addMatchHistoryToQueue(account_id, args.num_matches)
    while True:
        try:
            MatchReferenceDto = _match_queue.get(block = False)
            match = getMatch(MatchReferenceDto['gameId'])            
            recordMatch(account_id, MatchReferenceDto, match)
        except Empty:
            print("Empty match queue")
            exit()

def addMatchHistoryToQueue(account_id, num_matches):
    num_requests = max([1, ceil(num_matches / 100)])
    print("Getting match history for {}".format(account_id))
    for i in range(0, num_requests):
        params = {
                    'beginIndex': i * 100, 
                    'queue': _5v5_queue_ids
                }
        print("Queuing matches {} through {}".format(params['beginIndex'], params['beginIndex'] + 100))
        match_history = getSummonerMatchHistory(account_id, params) 
        for MatchReferenceDto in match_history['matches']:
            _match_queue.put(MatchReferenceDto)

def main():
    parser = argparse.ArgumentParser("Scrape Riot API data")
    parser.add_argument('--summoner_name', help="name of summoner for which to get match history", required=True)
    parser.add_argument('--num_matches', help="number of matches to crawl through", default=100, type=int)
    args = parser.parse_args()
    print('Getting {} matches for {}'.format(args.num_matches, args.summoner_name))
    initCSV()
    seed_summoner = getSummonerByName(args.summoner_name)
    crawl(args, seed_summoner['accountId'])

def exit():
    # ?
    sys.exit(0)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        exit()

atexit.register(atExit)
