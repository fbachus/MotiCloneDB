CREATE TABLE IF NOT EXISTS "User" (
	id SERIAL PRIMARY KEY,
	username VARCHAR(30) NOT NULL,
	email VARCHAR(60) UNIQUE NOT NULL,
	password VARCHAR(82) NOT NULL,
	salt VARCHAR(50) NOT NULL,
	weekly_workout_goal INTEGER NOT NULL DEFAULT '0'
);
