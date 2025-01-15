CREATE TABLE teacher_student (
    teacher_id VARCHAR(21) REFERENCES "user"(id) ON DELETE CASCADE,
    student_id VARCHAR(21) REFERENCES "user"(id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'active',
    markdown TEXT,
    joined TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (teacher_id, student_id)
);

CREATE INDEX idx_teacher_student_notes 
    ON teacher_student 
    USING GIN (to_tsvector('english', markdown));