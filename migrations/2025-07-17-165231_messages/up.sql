CREATE TABLE IF NOT EXISTS messages (
	id SERIAL PRIMARY KEY,
	author_id integer REFERENCES users (id) ON DELETE SET NULL,
	groups_id integer REFERENCES groups (id) ON DELETE CASCADE,
	content text NOT NULL,
	time timestamp NOT NULL,
	proof boolean
);
