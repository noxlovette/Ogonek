-- Add migration script here
CREATE TABLE activity_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR(21) NOT NULL REFERENCES "user"(id),
    model_type VARCHAR(50) NOT NULL, -- 'task', 'project', etc
    model_id VARCHAR(21) NOT NULL,
    action VARCHAR(20) NOT NULL, -- 'created', 'updated', 'deleted', 'completed'
    metadata JSONB, -- flexible data like old/new values
    created_at TIMESTAMPTZ DEFAULT NOW(),
    target_user_id VARCHAR(21) REFERENCES "user"(id)
);

-- Indexes for performance
CREATE INDEX idx_activity_user_created ON activity_logs(user_id, created_at DESC);
CREATE INDEX idx_activity_target_user ON activity_logs(target_user_id, created_at DESC);