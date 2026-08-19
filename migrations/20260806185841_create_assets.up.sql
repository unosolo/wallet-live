-- Add up migration script here
CREATE TABLE IF NOT EXISTS asset (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL,
    unit_value DOUBLE PRECISION NOT NULL
);
