CREATE TABLE IF NOT EXISTS users (
	id SERIAL PRIMARY KEY,
	username varchar(30) NOT NULL,
	email varchar(60) UNIQUE NOT NULL,
	password varchar(82) NOT NULL,
	weekly_workout_goal int
);
