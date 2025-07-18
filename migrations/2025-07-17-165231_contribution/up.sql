CREATE TABLE IF NOT EXISTS contribution (
	id SERIAL PRIMARY KEY,
	commitment integer REFERENCES commitment (id) ON DELETE RESTRICT,
	time timestamp NOT NULL,
	amount integer NOT NULL
);
-- Your SQL goes here
