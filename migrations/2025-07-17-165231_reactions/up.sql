CREATE TABLE IF NOT EXISTS reactions (
	user_id integer REFERENCES users (id) ON DELETE CASCADE,
	message_id integer REFERENCES messages (id) ON DELETE CASCADE,
	emoji varchar(8) NOT NULL
);
