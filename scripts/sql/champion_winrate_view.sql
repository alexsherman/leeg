-- champion winrate view
CREATE FUNCTION global_winrate(text)
	RETURNS decimal AS $$
		select distinct (select count(*) * 100.0 from all_matches where (blue_wins = true and blue_team @> array[$1]) or (blue_wins = false and red_team @> array[$1])) 
		/ 
		(SELECT count(*) from all_matches where (blue_team || red_team) @> array[$1]) as ezreal_winrate from all_matches;
	$$ LANGUAGE SQL;

CREATE FUNCTION recent_patch_winrate(text)
        RETURNS decimal AS $$
	select distinct (select count(*) * 100.0 from all_matches where ((blue_wins = true and blue_team @> array[$1]) or (blue_wins = false and red_team @> array[$1])) and game_version = (select most_recent_patch()))
	/
	(SELECT count(*) from all_matches where (blue_team || red_team) @> array[$1] and game_version = (select most_recent_patch())) as wr from all_matches;
	$$ LANGUAGE SQL;


CREATE MATERIALIZED VIEW champion_winrates
AS
select name, global_winrate(name) as winrate from (SELECT DISTINCT UNNEST(red_team || blue_team)
AS name
FROM all_matches LIMIT 150) as names ORDER BY winrate desc;

-- create a unique index on the view so that it can be queried even while being refreshed
CREATE UNIQUE INDEX champion_name_index on champion_winrates (name);

-- when desired, refresh the view
REFRESH MATERIALIZED VIEW CONCURRENTLY champion_winrates;

CREATE FUNCTION global_num_games(text)
	RETURNS bigint AS $$
	select count(*) from all_matches where red_team @> array[$1] or blue_team @> array[$1];
	$$ LANGUAGE SQL;

CREATE FUNCTION global_pick_rate(text)
	RETURNS decimal AS $$
	select global_num_games($1)::decimal / (select count(*) from all_matches);
	$$ LANGUAGE SQL;

-- to do - make this only use most recent big patch (i.e. 9.9) instead of more granular
CREATE FUNCTION most_recent_patch()
	RETURNS text AS $$
	select game_version from all_matches ORDER BY play_date DESC limit 1;
	$$ LANGUAGE SQL;
	
CREATE FUNCTION recent_patch_pick_rate(text)
        RETURNS decimal AS $$
	select recent_patch_num_games($1)::decimal / (select count(*) from all_matches where game_version = (select most_recent_patch()));
	$$ LANGUAGE SQL;

CREATE FUNCTION recent_patch_num_games(text)
        RETURNS bigint AS $$
	select count(*) from all_matches where (red_team @> array[$1] or blue_team @> array[$1]) and game_version = (select most_recent_patch());
	$$ LANGUAGE SQL;

CREATE MATERIALIZED VIEW wr_pr_most_recent_patch
AS
select name, recent_patch_winrate(name) as winrate, recent_patch_pick_rate(name) as pickrate, recent_patch_num_games(name) as totalgames from (SELECT DISTINCT UNNEST(red_team || blue_team)
	AS name
	FROM all_matches LIMIT 150) as names ORDER BY winrate desc;

CREATE UNIQUE INDEX champion_name_index_recent_patch on wr_pr_most_recent_patch(name);
