CREATE TABLE IF NOT EXISTS commitment (
	id SERIAL PRIMARY KEY,
	user_id integer REFERENCES users (id) ON DELETE CASCADE,
	groups_id integer REFERENCES groups (id) ON DELETE CASCADE,
	amount integer
);

