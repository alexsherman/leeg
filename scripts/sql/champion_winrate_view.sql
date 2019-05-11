CREATE FUNCTION global_winrate(text)
	RETURNS decimal AS $$
		select distinct (select count(*) * 100.0 from all_matches where (blue_wins = true and blue_team @> array[$1]) or (blue_wins = false and red_team @> array[$1])) 
		/ 
		(SELECT count(*) from all_matches where (blue_team || red_team) @> array[$1]) as ezreal_winrate from all_matches;
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

