CREATE TABLE tasks (
    -- Assuming you have or will create a ULID type or use TEXT for ULID storage
    id TEXT PRIMARY KEY,  -- ULID type doesn't exist in standard PostgreSQL; using TEXT as a placeholder for ULID
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    priority SMALLINT NOT NULL DEFAULT 1 CHECK (priority >= 1 AND priority <= 3),
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMP WITH TIME ZONE,
    file VARCHAR(255),
    assignee_id UUID NOT NULL,
    FOREIGN KEY (assignee_id) REFERENCES users(id) ON DELETE CASCADE
);