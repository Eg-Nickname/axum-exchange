-- Add migration script here
CREATE TABLE IF NOT EXISTS item_offers (
    id bigserial PRIMARY KEY UNIQUE,
    author_id BIGINT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE,
    item_id BIGINT NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    packets BIGINT NOT NULL,
    items_per_packet BIGINT NOT NULL,
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
