-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    sender_id SERIAL REFERENCES users(id),
    channel_id SERIAL REFERENCES channels(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
