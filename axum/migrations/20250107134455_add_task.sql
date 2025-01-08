-- Add migration script here

CREATE TABLE tasks (
    id VARCHAR(21) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    markdown TEXT NOT NULL,
    priority SMALLINT NOT NULL DEFAULT 1 
        CHECK (priority >= 1 AND priority <= 3),
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    file_path TEXT,
    created_by VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    assignee VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE
);

-- Add trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tasks_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();