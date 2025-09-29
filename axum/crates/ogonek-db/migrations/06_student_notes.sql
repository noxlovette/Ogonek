CREATE TABLE student_notes (
    id VARCHAR(21) PRIMARY KEY,
    lesson_id VARCHAR(21) NOT NULL,
    user_id VARCHAR(21) NOT NULL,
    is_bookmarked BOOLEAN DEFAULT false,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lesson_id) REFERENCES lessons(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
    CONSTRAINT unique_student_lesson_note UNIQUE (lesson_id, user_id)
);

CREATE INDEX idx_student_notes_lesson_user 
    ON student_notes(lesson_id, user_id);
