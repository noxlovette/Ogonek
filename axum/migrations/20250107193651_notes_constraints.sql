-- Add unique constraint for lesson_id and user_id combination
ALTER TABLE student_notes
    ADD CONSTRAINT unique_student_lesson_note 
    UNIQUE (lesson_id, user_id);

-- Add indexes to improve query performance
CREATE INDEX idx_student_notes_lesson_user 
    ON student_notes(lesson_id, user_id);