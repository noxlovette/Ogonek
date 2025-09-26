-- Add migration script here
BEGIN;

ALTER TABLE teacher_student
RENAME COLUMN telegram_id TO student_telegram_id;

ALTER TABLE teacher_student
ADD COLUMN teacher_telegram_id VARCHAR(20),
ADD COLUMN teacher_zoom_url VARCHAR(255);

CREATE INDEX idx_teacher_student_status ON teacher_student (status);

COMMIT;
