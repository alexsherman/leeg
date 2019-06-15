import json
import datetime
import argparse
from db_utils import *
import psycopg2
from spider_utils import *

def insertChampionMastery(champion_mastery_dto, cursor):
    sql = '''
                INSERT INTO summoner_masteries
                (summoner_id, champion_id, mastery_level, mastery_points, last_played)
                VALUES (%s, %s, %s, %s, %s)
                ON CONFLICT (summoner_id, champion_id) DO UPDATE
                SET mastery_level = excluded.mastery_level,
                    mastery_points = excluded.mastery_points,
                    last_played = excluded.last_played
            '''
    last_played = datetime.datetime.fromtimestamp(champion_mastery_dto['lastPlayTime'] / 1000)  
    sql_tuple = (champion_mastery_dto['summonerId'], champion_mastery_dto['championId'], 
                 champion_mastery_dto['championLevel'], champion_mastery_dto['championPoints'], 
                 last_played)
    try:
        cursor.execute(sql, sql_tuple)
    except psycopg2.DatabaseError as e:
        print(e)


if __name__ == '__main__':
    parser = argparse.ArgumentParser("Scrape Riot API data")
    parser.add_argument('--summoner_name', help="name of summoner for which to get masteries", required=True)
    args = parser.parse_args()
    seed_summoner = getSummonerByName(args.summoner_name)
    masteries = getSummonerMasteries(seed_summoner['id'])
    db = connect()
    for mastery_dto in masteries:
        insertChampionMastery(mastery_dto, db['cursor'])
    print('success')
    db['connection'].commit()
    db['connection'].close()
    db['cursor'].close()