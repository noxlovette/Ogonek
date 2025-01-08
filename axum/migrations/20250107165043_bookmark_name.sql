-- Add migration script here
-- Rename the table
ALTER TABLE lesson_notes RENAME TO bookmarks;

-- Rename indexes
ALTER INDEX idx_lesson_notes_lesson_id RENAME TO idx_bookmarks_lesson_id;
ALTER INDEX idx_lesson_notes_user_id RENAME TO idx_bookmarks_user_id;

-- Rename the unique constraint
ALTER TABLE bookmarks RENAME CONSTRAINT unique_lesson_user TO unique_bookmark_lesson_user;

-- Drop the old trigger
DROP TRIGGER update_lesson_notes_updated_at ON bookmarks;

-- Create new trigger with updated name
CREATE TRIGGER update_bookmarks_updated_at
    BEFORE UPDATE ON bookmarks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();