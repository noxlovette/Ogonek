-- Add migration script here
CREATE TABLE student_notes (
    id VARCHAR(21) PRIMARY KEY,
    lesson_id VARCHAR(21) NOT NULL,
    user_id VARCHAR(21) NOT NULL,
    is_bookmarked BOOLEAN DEFAULT false,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (lesson_id) REFERENCES lessons(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE
);