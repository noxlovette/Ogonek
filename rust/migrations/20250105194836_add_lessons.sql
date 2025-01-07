-- Add migration script here
CREATE TABLE IF NOT EXISTS lessons (
   id VARCHAR(21) PRIMARY KEY,
   markdown TEXT NOT NULL DEFAULT '#New Note',
   title VARCHAR(255) NOT NULL,
   topic VARCHAR(255) NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   created_by VARCHAR(21) NOT NULL REFERENCES "user"(id),
   assignee VARCHAR(21) NOT NULL REFERENCES "user"(id)
);

-- Trigger to automatically update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_lessons_updated_at
    BEFORE UPDATE ON lessons
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();