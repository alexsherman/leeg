import os
import sys
import json
import csv
import argparse
import atexit
from queue import Queue, Empty
from spider_utils import *
from spider_classes import *

_matches_processed = 0
_match_queue = Queue()
_match_ids_processed = set()
_summoner_ids_processed = set()
_5v5_queue_ids = [400, 420]
#TODO make constants file

# TODO decide on formats and init various csvs accordingly
def initCSV():
    with open('matches.csv', mode='w+') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        match_writer.writerow(['match_id', 'winning_team', 'champsAndPlayers', 'gameVersion'])
    
    with open('matches_short.csv', mode='w+') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        match_writer.writerow(['match_id', 'winning_team', 'blueTeam', 'redTeam', 'gameVersion'])
    

def recordMatch(match):
    '''
    Given MatchDto, converts it to Match instance and appends it to csv.
    TODO: various csvs, allow command line arg specify file

    Args:
        match: MatchDto
    '''
    m = Match(match)
    
    with open('matches.csv', mode='a') as match_csv:
        match_writer =  csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        print('Writing match ' + str(m.id))
        m.write(match_writer)

        with open('matches_short.csv', mode='a+') as match_short_csv:
            match_writer = csv.writer(match_short_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
            m.writeNamesAndLanes(match_writer)

        global _matches_processed
        _matches_processed += 1
        _match_ids_processed.add(m.id)

def crawl(args, account_id):
    '''
    Given command line args and summoner id, begins crawling through matches and recording them.
    Args:
        args: command line args parsed. determine number of matches until quit. TODO rename
        account_id       

    '''
    addMatchHistoryToQueue(account_id, args.match_depth)
    while True:
        try:
            match_id = _match_queue.get(block = False)
            if match_id in _match_ids_processed:
                print("Match {} already recorded by this spider".format(match_id))
                continue
            print(match_id)
            match = getMatch(match_id)
            if match['queueId'] not in _5v5_queue_ids:
                continue

            recordMatch(match)
            champs = map(lambda p: p['championId'], match['participants'])
            print(map(lambda c: champ_dict[str(c)], champs))

            if _matches_processed >= args.n_matches:
                print("Successfully processed {} matches, exiting".format(_matches_processed))
                exit()

            if _match_queue.qsize() > 5:
                continue;
        
            summonerIds = map(lambda p: p['accountId'], Match(match).playersAndChamps)
            for id in summonerIds:
                if id not in _summoner_ids_processed:
                    _summoner_ids_processed.add(id)
                    addMatchHistoryToQueue(id, args.match_depth)
        except Empty:
            print("Empty match queue")
            exit()

def addMatchHistoryToQueue(account_id, match_depth):
    print("Getting match history for {}".format(account_id))
    match_history = getSummonerMatchHistory(account_id, {'endIndex': match_depth})
    match_ids = getMatchIds(match_history)
    for match_id in match_ids:
        _match_queue.put(match_id)

def main():
    parser = argparse.ArgumentParser("Scrape Riot API data")
    parser.add_argument('--summoner_name', help="name of summoner, seed for spider", default="sleepo mode")
    parser.add_argument('--n_matches', help="number of matches to crawl through", default=1000, type=int)
    parser.add_argument('--match_depth', help="number of matches per summoner match history to process", default=5, type=int)
    args = parser.parse_args()
    initCSV()
    seed_summoner = getSummonerByName(args.summoner_name)
    crawl(args, seed_summoner['accountId'])

def atExit():
    with open('spider_report.txt', 'a+') as r:
        r.write("Matches processed: {}\r\n".format(_matches_processed))
        r.write("Match ids processed: \r\n")
        for match_id in _match_ids_processed:
            r.write(str(match_id) + '\r\n')

def exit():
    atExit()
    sys.exit(0)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        print('Interrupted manually, generating report')
        exit()

atexit.register(atExit)
