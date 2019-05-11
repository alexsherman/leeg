from configparser import ConfigParser
import psycopg2

def config(filename='database.ini', section='postgresql'):
    parser = ConfigParser()
    parser.read(filename)
    db = {}
    if parser.has_section(section):
        params = parser.items(section)
        for param in params:
            db[param[0]] = param[1]
    else:
        raise Exception('database.ini file missing postgres section')
    
    return db

def connect():
    '''
    Returns a dict with connection and cursor to the postgres database specified in database.ini.
    
    Client calling connect() is responsible for calling .close() on both cursor and connection.
    '''
    connection = None
    try:
        params = config()
        print('Connecting to {} as {}...'.format(params['host'], params['user']))    
        connection = psycopg2.connect(**params)
        cursor = connection.cursor()
        return {
            'connection': connection,
            'cursor': cursor
        }

    except (Exception, psycopg2.DatabaseError) as error:
        print(error)
        if connection is not None:
            connection.close()


