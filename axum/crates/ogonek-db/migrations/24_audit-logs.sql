CREATE TABLE audit_logs (
    id VARCHAR(21) PRIMARY KEY,
    
    -- Core event info
    event_type VARCHAR NOT NULL, -- 'user.login', 'content.update', 'lesson.delete'
    action VARCHAR NOT NULL, -- 'CREATE', 'UPDATE', 'DELETE', 'ACCESS'
    outcome VARCHAR NOT NULL DEFAULT 'success', -- 'success', 'failure', 'partial'
    
    user_id VARCHAR(21) REFERENCES "user"(id) ON DELETE SET NULL, -- SET NULL, not CASCADE
    user_email VARCHAR NOT NULL, -- always snapshot this
    user_role VARCHAR NOT NULL,
    impersonated_by VARCHAR(21) REFERENCES "user"(id) ON DELETE SET NULL,
    
    -- Target
    resource_type VARCHAR NOT NULL,
    resource_id VARCHAR,
    resource_name VARCHAR, -- human readable
    
    -- The money shot - flexible change tracking
    payload JSONB NOT NULL DEFAULT '{}', -- all your context data here
    
    -- Request context  
    session_id VARCHAR,
    ip_address INET,
    user_agent TEXT,
    request_id VARCHAR, -- for tracing
    
    -- Metadata
    severity VARCHAR NOT NULL DEFAULT 'info',
    tags TEXT[],
    
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TIMESTAMPTZ
);