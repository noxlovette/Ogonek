-- Add migration script here
ALTER TABLE files
ADD COLUMN IF NOT EXISTS upload_status TEXT DEFAULT 'complete';

CREATE INDEX IF NOT EXISTS idx_files_upload_status ON files (upload_status);

UPDATE files
SET
    upload_status = 'complete'
WHERE
    upload_status IS NULL;

ALTER TABLE files ADD CONSTRAINT check_upload_status CHECK (upload_status IN ('pending', 'complete', 'error'));
