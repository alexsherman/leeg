CREATE TABLE summoner_masteries (
    summoner_id text, 
    champion_id smallint NOT NULL,
    mastery_level smallint NOT NULL,
    mastery_points int NOT NULL,
    last_played timestamp,
    PRIMARY KEY (summoner_id, champion_id)
);

GRANT INSERT, UPDATE, SELECT on summoner_masteries to spider;
GRANT SELECT on summoner_masteries to api;

