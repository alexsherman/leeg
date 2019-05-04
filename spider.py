import os
import sys
import json
import csv
import argparse
import atexit
from spider_utils import *
from spider_classes import *

_matches_processed = 0
_match_ids_processed = []

def initCSV():
    with open('matches.csv', mode='w') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        match_writer.writerow(['match_id', 'winning_team', 'champsAndPlayers', 'team2champs', 'gameVersion'])

def recordMatch(match):
    m = Match(match)
    if m.id in _match_ids_processed:
        print("Match already recorded by this spider")
        return

    with open('matches.csv', mode='a') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        print('Writing match ' + str(m.id))
        m.write(match_writer)
        global _matches_processed
        _matches_processed += 1
        _match_ids_processed.append(m.id)

def main():
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
        print(map(lambda c: champ_dict[str(c)], champs))
        recordMatch(match)

def atExit():
    with open('spider_report.txt', 'a+') as r:
        r.write("Matches processed: {}\r\n".format(_matches_processed))
        r.write("Match ids processed: \r\n")
        for match_id in _match_ids_processed:
            r.write(str(match_id) + '\r\n')

if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        print('Interrupted manually, generating report')
        atExit()
        try:
            sys.exit(0)
        except:
            os._exit(0)
       
    
atexit.register(atExit)
