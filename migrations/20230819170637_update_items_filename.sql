-- Add migration script here
UPDATE items SET filename = Replace(filename, ' ', '_');