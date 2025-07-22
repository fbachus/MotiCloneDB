CREATE TABLE IF NOT EXISTS "UserWeeklyTarget" (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES "User" (id) ON DELETE CASCADE,
	groupchallenge_id INTEGER REFERENCES "GroupChallenge" (
		challenge_id
	) ON DELETE SET NULL,
	target_count INTEGER NOT NULL,
	achieved_count INTEGER NOT NULL,
	current_progress VARCHAR(255),
	created_at TIMESTAMP
);
