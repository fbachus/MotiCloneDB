CREATE TABLE IF NOT EXISTS workouts (
	user_id integer REFERENCES users (id) NOT NULL ON DELETE CASCADE,
	proof_path integer REFERENCES messages (id) NOT NULL ON DELETE CASCADE,
	verified boolean
);
