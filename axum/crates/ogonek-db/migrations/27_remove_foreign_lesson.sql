-- Add migration script here
ALTER TABLE lessons 
ALTER COLUMN assignee DROP NOT NULL;

ALTER TABLE lessons 
ADD COLUMN template_id VARCHAR(21) REFERENCES lessons(id) ON DELETE SET NULL;

CREATE INDEX idx_lessons_template_id ON lessons(template_id);