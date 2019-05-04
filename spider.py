import requests
import os
import time
import json
import csv
import argparse
from spider_utils import *
from spider_classes import *


def initCSV():
    with open('matches.csv', mode='w') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        match_writer.writerow(['match_id', 'winning_team', 'champsAndPlayers', 'team2champs', 'gameVersion'])

def recordMatch(match):
    with open('matches.csv', mode='a') as match_csv:
        match_writer = csv.writer(match_csv, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL)
        m = Match(match)
        print('Writing match ' + str(m.id))
        m.write(match_writer) 
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
      #  print(champs)
      #  print(map(lambda c: champ_dict[str(c)], champs))
        recordMatch(match)
    
