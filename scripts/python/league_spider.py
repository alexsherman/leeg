import os
import sys
import json
import csv
import argparse
import atexit
from requests.exceptions import HTTPError
from math import ceil
from queue import Queue, Empty
from spider_utils import *
from spider_classes import *
from db_utils import *

_match_queue = Queue()
_5v5_queue_ids = [420] #only ranked 5v5
_num_processed = 0
_summoner_ids_processed = set()
_db = {}

def recordMatch(match, tier):    
    m = Match(match, tier)
    m.insert_into_all_matches(_db['cursor'])
    global _num_processed
    _num_processed += 1
    _db['connection'].commit()
    if _num_processed % 10 == 0:
        
        print("Processed {} matches.".format(_num_processed))

def crawl():
    # generator yields a bunch of matches endlessly
    for matches in all_matches_today(1):
        print("{} more matches to process".format(len(matches)))
        for matchdto in matches: 
            match = getMatch(matchdto['gameId'])
            recordMatch(match, matchdto['approximateTier'])

def main():
    global _db
    _db = connect() 
    crawl()

def exit():
    _db['connection'].close()
    _db['cursor'].close()
    sys.exit(0)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        exit()