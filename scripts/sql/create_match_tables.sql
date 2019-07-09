CREATE TABLE all_matches (
	id bigint PRIMARY KEY, 
	blue_wins boolean NOT NULL,
	blue_team text[] NOT NULL, -- the following are all arrays of text representing champion names
	red_team text[] NOT NULL,
	blue_bans text[],
	red_bans text[],
	red_summoners text[] NOT NULL, --account ids - order same as x_team order
	blue_summoners text[] NOT NULL,
	play_date timestamp,
	duration interval,
	game_version text,
	game_mode integer
);
 
CREATE UNIQUE INDEX idx_match_id ON all_matches(id);
CREATE INDEX idx_champions ON all_matches USING GIN(blue_team, red_team, blue_bans, red_bans);

CREATE TABLE summoner_matches (
	id text,
	name text NOT NULL,
	wins boolean NOT NULL,
	champion text NOT NULL,
	same_team_champions text[],
	opp_team_champions text[],
	same_team_bans text[],
	opp_team_bans text[],
	match_id bigint,
	play_date timestamp,
	game_version text,
	PRIMARY KEY (id, match_id),
	CONSTRAINT summoner_match_id_fkey FOREIGN KEY (match_id)
		REFERENCES all_matches (id) MATCH SIMPLE
		ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE UNIQUE INDEX idx_summoner_id_match_id ON summoner_matches(id, match_id);

-- NEW 
CREATE TABLE all_matches_2 (
	id bigint PRIMARY KEY, 
	play_date timestamp,
	blue_wins boolean NOT NULL,
	blue_team smallint[] NOT NULL, -- the following are all arrays of text representing champion names
	red_team smallint[] NOT NULL,
	blue_bans smallint[] NOT NULL,
	red_bans smallint[] NOT NULL,
	red_roles text[] NOT NULL, --account ids - order same as x_team order
	blue_roles text[] NOT NULL,
	rank text,
	game_version text
);

CREATE INDEX idx_champions_in_match ON all_matches_2 USING GIN(blue_team, red_team);
CREATE INDEX idx_banned_champions ON all_matches_2(red_bans, blue_bans);
CREATE INDEX idx_rank_of_match ON all_matches_2(rank);
CREATE INDEX idx_playdate_of_match on all_matches_2(play_date);
CREATE INDEX idx_version_of_match on all_matches_2(game_version);

GRANT SELECT, INSERT, UPDATE, DELETE ON all_matches_2 to spider;
GRANT SELECT ON all_matches_2 to api;
