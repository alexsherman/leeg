import os
import sys
import json
import csv
import argparse
import atexit
from rquests.exceptions import HTTPError
from math import ceil
from queue import Queue, Empty
from spider_utils import *
from spider_classes import *
from db_utils import *

_match_queue = Queue()
_5v5_queue_ids = [400, 420]
_num_processed = 0
_db = {}

def recordMatch(account_id, MatchReferenceDto, match):    
    m = Match(match)
    m.insert_into_all_matches(_db['cursor'])
    _db['connection'].commit()
    s = Summoner().from_match(m, account_id)
    m.insert_into_summoner_matches(_db['cursor'], s)
    _db['connection'].commit()
    global _num_processed
    _num_processed += 1
    if _num_processed % 10 == 0:
        print("Processed {} matches.".format(_num_processed))

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
        except HTTPError as e:
            continue

def addMatchHistoryToQueue(account_id, num_matches):
    num_requests = max([1, ceil(num_matches / 100)])
    print("Getting match history for {}".format(account_id))
    #TODO - query db to check which matches we already have, remove those from queue
    sql = '''
            SELECT id from summoner_matches where id = %s
        '''
    sql_tuple = (account_id,)
    print(sql, sql_tuple)
    ids = _db['cursor'].execute(sql, sql_tuple)
    print(ids)
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
    global _db
    _db = connect() 
    seed_summoner = getSummonerByName(args.summoner_name)
    crawl(args, seed_summoner['accountId'])

def exit():
    _db['connection'].close()
    _db['cursor'].close()
    sys.exit(0)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        exit()

atexit.register(atExit)
