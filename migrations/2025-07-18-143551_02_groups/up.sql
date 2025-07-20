CREATE TABLE IF NOT EXISTS groups (
    id SERIAL PRIMARY KEY,
    referral_code VARCHAR(15) UNIQUE,
    admin INTEGER REFERENCES users (id) NOT NULL,
    name VARCHAR(255) NOT NULL,
    challenge_deadline TIMESTAMP
);
