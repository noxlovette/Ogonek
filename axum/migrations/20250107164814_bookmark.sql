-- Add migration script here
-- Add lesson_notes table
CREATE TABLE IF NOT EXISTS lesson_notes (
    -- Primary key using nanoid
    id VARCHAR(21) PRIMARY KEY,
    
    -- Foreign key to lessons table
    lesson_id VARCHAR(21) NOT NULL,
    CONSTRAINT fk_lesson
        FOREIGN KEY(lesson_id)
        REFERENCES lessons(id)
        ON DELETE CASCADE,
    
    -- Foreign key to users table
    user_id VARCHAR(21) NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE,
    
    -- Notes field
    notes TEXT,
    
    -- Timestamps for record keeping
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure one-to-one relationship between lesson and user
    CONSTRAINT unique_lesson_user
        UNIQUE(lesson_id, user_id)
);

-- Create index for faster lookups
CREATE INDEX idx_lesson_notes_lesson_id ON lesson_notes(lesson_id);
CREATE INDEX idx_lesson_notes_user_id ON lesson_notes(user_id);

-- Add trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_lesson_notes_updated_at
    BEFORE UPDATE ON lesson_notes
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();