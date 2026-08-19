-- Add up migration script here

-- Create table user in PostgreSQL if not exists
CREATE TABLE IF NOT EXISTS "user" (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
