-- Add migration script here
CREATE TABLE task_files (
    task_id VARCHAR(21) NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    file_id VARCHAR(21) NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, file_id)
);

CREATE INDEX idx_task_files_task_id ON task_files(task_id);
CREATE INDEX idx_task_files_file_id ON task_files(file_id);