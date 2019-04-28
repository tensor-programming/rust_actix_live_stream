-- Your SQL goes here
CREATE TABLE messages (
    id UUID PRIMARY KEY NOT NULL,
    username VARCHAR(10) NOT NULL,
    body TEXT NOT NULL,
    ts TIMESTAMP NOT NULL
);