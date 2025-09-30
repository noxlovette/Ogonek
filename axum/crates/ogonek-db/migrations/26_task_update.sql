ALTER TABLE tasks 
ALTER COLUMN assignee DROP NOT NULL;

ALTER TABLE tasks 
ADD COLUMN template_id VARCHAR(21) REFERENCES tasks(id) ON DELETE SET NULL;

CREATE INDEX idx_tasks_template_id ON tasks(template_id);