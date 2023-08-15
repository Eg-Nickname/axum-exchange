-- Add migration script here
CREATE TABLE IF NOT EXISTS items (
    id bigserial PRIMARY KEY UNIQUE,
    item_name TEXT NOT NULL,
    minecraft_item_id TEXT NOT NULL,
    item_meta TEXT NOT NULL,
    has_NBT BOOL NOT NULL,
    display_name_eng TEXT NOT NULL,
    display_name_pl TEXT NOT NULL,
    filename TEXT NOT NULL
);
