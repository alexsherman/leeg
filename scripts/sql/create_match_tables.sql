CREATE TABLE all_matches (
	id bigint PRIMARY KEY, 
	blue_wins boolean NOT NULL,
	blue_team text[] NOT NULL, -- the following are all arrays of text representing champion names
	red_team text[] NOT NULL,
	blue_bans text[],
	red_bans text[],
	red_summoners int[] NOT NULL, --account ids - order same as x_team order
	blue_summoners int[] NOT NULL,
	play_date timestamp,
	duration interval,
	game_version text,
	game_mode integer
);
 
CREATE UNIQUE INDEX idx_match_id ON all_matches(id);

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


