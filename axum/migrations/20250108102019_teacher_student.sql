-- Add migration script here
CREATE TABLE teacher_student (
    teacher_id VARCHAR(21) REFERENCES "user"(id) ON DELETE CASCADE,
    student_id VARCHAR(21) REFERENCES "user"(id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'active',
    joined TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (teacher_id, student_id)
);

CREATE TABLE teacher_notes (
    teacher_id VARCHAR(21),
    student_id VARCHAR(21),
    markdown TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (teacher_id, student_id),
    FOREIGN KEY (teacher_id, student_id) REFERENCES teacher_student(teacher_id, student_id) ON DELETE CASCADE
);