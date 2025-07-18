CREATE TABLE IF NOT EXISTS contributions (
	id SERIAL PRIMARY KEY,
	commitment INTEGER REFERENCES commitments (id) ON DELETE RESTRICT,
	time TIMESTAMP NOT NULL,
	amount INTEGER NOT NULL
);
-- Your SQL goes here
