CREATE TABLE IF NOT EXISTS workouts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    proof_path INTEGER NOT NULL,
    verified BOOLEAN,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (proof_path) REFERENCES messages (id) ON DELETE CASCADE
);
