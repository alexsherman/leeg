from db_utils import *
import psycopg2
import json

# Crude script just to insert the json files we had into db

def insert_champions():
    with open('../../champions.json') as champs:
        with open('../../champion_roles.json') as cr:
            _db = connect() 
            champ_dict = json.load(champs)
            champ_roles = json.load(cr)
            for idx, name in champ_dict.items():
                sql = '''INSERT INTO champions
                        (id, name, roles)
                        VALUES (%s, %s, %s)
                '''
                sql_tuple = (int(idx), name, champ_roles[idx])
                try:
                    _db['cursor'].execute(sql, sql_tuple)
                    print(sql_tuple)
                except psycopg2.DatabaseError as e:
                    print(e)
            _db['connection'].commit()
            _db['connection'].close()

if __name__ == '__main__':
    insert_champions()