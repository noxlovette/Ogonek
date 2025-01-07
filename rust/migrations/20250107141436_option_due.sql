-- Add migration script here
ALTER TABLE tasks
    ALTER COLUMN due_date DROP NOT NULL;