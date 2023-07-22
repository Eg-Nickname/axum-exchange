-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY UNIQUE,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);