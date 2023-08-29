-- Your SQL goes here
CREATE TABLE channels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(1024) NOT NULL,
    owner_id SERIAL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE channels_users (
    channel_id SERIAL REFERENCES channels(id),
    user_id SERIAL REFERENCES users(id),
    PRIMARY KEY(channel_id, user_id)
);
