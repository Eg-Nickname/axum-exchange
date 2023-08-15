-- Add migration script here
CREATE TABLE IF NOT EXISTS colors (
    id bigserial PRIMARY KEY UNIQUE,
    color CUBE NOT NULL,
    color_index INT NOT NULL,
    item_id BIGINT NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
);