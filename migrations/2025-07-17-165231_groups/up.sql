CREATE TABLE IF NOT EXISTS groups (
	id SERIAL PRIMARY KEY,
	referral_code varchar(15) UNIQUE,
	admin integer REFERENCES users (id) NOT NULL,
	name varchar(255) NOT NULL,
	challenge_deadline timestamp
);
