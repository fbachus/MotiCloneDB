CREATE TABLE IF NOT EXISTS users (
	id SERIAL PRIMARY KEY,
	username varchar(30) NOT NULL,
	email varchar(60) CHECK UNIQUE NOT NULL,
	password varchar(82) NOT NULL,
	weekly_workout_goal int
);

CREATE TABLE IF NOT EXISTS groups (
	id SERIAL PRIMARY KEY,
	referral_code varchar(15) UNIQUE,
	admin integer REFERENCES users (id) NOT NULL,
	name varchar(255) NOT NULL,
	challenge_deadline timestamp
);

CREATE TABLE IF NOT EXISTS commitment (
	id SERIAL PRIMARY KEY,
	user_id integer REFERENCES users (id) ON DELETE CASCADE,
	groups_id integer REFERENCES groups (id) ON DELETE CASCADE,
	amount integer
);

CREATE TABLE IF NOT EXISTS pool (
	id SERIAL PRIMARY KEY,
	manager integer REFERENCES users (id) ON DELETE RESTRICT,
	purpose varchar(255),
	amount integer NOT NULL
);

CREATE TABLE IF NOT EXISTS contribution (
	id SERIAL PRIMARY KEY,
	commitment integer REFERENCES commitment (id) ON DELETE RESTRICT,
	time timestamp NOT NULL,
	amount integer NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
	id SERIAL PRIMARY KEY,
	author_id integer REFERENCES users (id) ON DELETE SET NULL,
	groups_id integer REFERENCES groups (id) ON DELETE CASCADE,
	content text NOT NULL,
	time timestamp NOT NULL,
	proof boolean
);

CREATE TABLE IF NOT EXISTS workouts (
	user_id integer REFERENCES users (id) NOT NULL ON DELETE CASCADE,
	proof_path integer REFERENCES messages (id) NOT NULL ON DELETE CASCADE,
	verified boolean
);

CREATE TABLE IF NOT EXISTS reactions (
	user_id integer REFERENCES users (id) ON DELETE CASCADE,
	message_id integer REFERENCES messages (id) ON DELETE CASCADE,
	emoji varchar(8) NOT NULL
);
