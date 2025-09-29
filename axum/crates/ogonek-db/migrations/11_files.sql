-- Add migration script here
CREATE TABLE files (
    id VARCHAR(21) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    s3_key VARCHAR(255) NOT NULL,
    path VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100),
    size BIGINT NOT NULL,
    is_folder BOOLEAN NOT NULL DEFAULT FALSE,
    parent_id VARCHAR(21) REFERENCES files(id) ON DELETE CASCADE,
    owner_id VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    visibility VARCHAR(20) NOT NULL DEFAULT 'private',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT valid_visibility CHECK (visibility IN ('private', 'shared', 'public'))
);

-- Junction table for file sharing
CREATE TABLE file_shares (
    file_id VARCHAR(21) NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    user_id VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    permission VARCHAR(20) NOT NULL DEFAULT 'view', -- 'view', 'edit', 'manage'
    shared_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    shared_by VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    PRIMARY KEY (file_id, user_id)
);

CREATE INDEX idx_files_parent_id ON files(parent_id);
CREATE INDEX idx_files_owner_id ON files(owner_id);
CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_file_shares_user_id ON file_shares(user_id);
