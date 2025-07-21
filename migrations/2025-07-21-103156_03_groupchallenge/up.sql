CREATE TABLE IF NOT EXISTS "GroupChallenge" (
	challenge_id SERIAL PRIMARY KEY,
	group_id INTEGER NOT NULL REFERENCES "Group" (id) ON DELETE CASCADE,
	description VARCHAR(1000) NOT NULL,
	start_date DATE NOT NULL,
	end_date DATE NOT NULL,
	created_at TIMESTAMP NOT NULL,
	updated_at TIMESTAMP NOT NULL
);
