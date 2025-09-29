CREATE TABLE content (
    id VARCHAR(21) PRIMARY KEY,
    slug VARCHAR NOT NULL UNIQUE, -- 'privacy-policy', 'about-us', etc
    title VARCHAR NOT NULL, -- for <title> tags and breadcrumbs
    markdown TEXT NOT NULL,
    meta_description TEXT, -- SEO juice
    version INTEGER NOT NULL DEFAULT 1,
    status VARCHAR NOT NULL DEFAULT 'draft', -- 'draft', 'published'
    published_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE RESTRICT
);

-- Indexes for your common queries
CREATE INDEX idx_content_slug ON content(slug);
CREATE INDEX idx_content_status ON content(status);
CREATE INDEX idx_content_published_at ON content(published_at DESC);