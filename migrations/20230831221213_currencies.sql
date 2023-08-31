-- Add migration script here
CREATE TABLE IF NOT EXISTS currencies (
    id bigserial PRIMARY KEY UNIQUE,
    name TEXT NOT NULL,
    code TEXT NOT NULL
);
