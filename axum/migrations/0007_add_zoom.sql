-- Add migration script here
ALTER TABLE profile
ADD COLUMN zoom_url VARCHAR(255),
ADD CONSTRAINT valid_zoom_url CHECK (zoom_url ~ '^https?:\/\/(?:[a-z0-9-]+\.)?zoom\.us\/j\/\d{9,11}(?:\?pwd=[a-zA-Z0-9]+)?$');