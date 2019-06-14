-- short and sweet
CREATE TABLE champions (
    id smallint PRIMARY KEY, 
    name text NOT NULL,
    roles text[],
    aliases text[]
);