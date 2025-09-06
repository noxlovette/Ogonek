-- Add migration script here
CREATE TABLE photos (
   id VARCHAR(21) PRIMARY KEY,
   unsplash_id VARCHAR(50) UNIQUE NOT NULL,
   urls JSONB NOT NULL,
   alt_description TEXT,
   photographer_name VARCHAR(255) NOT NULL,
   photographer_username VARCHAR(255) NOT NULL
);

CREATE INDEX idx_photos_unsplash_id ON photos(unsplash_id);

ALTER TABLE lessons ADD COLUMN photo_id VARCHAR(21) 
REFERENCES photos(id) ON DELETE SET NULL;