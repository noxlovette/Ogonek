-- Drop the old constraint if it exists
ALTER TABLE profile
DROP CONSTRAINT IF EXISTS valid_zoom_url;

-- Rename the column
ALTER TABLE profile
RENAME COLUMN zoom_url TO video_call_url;

-- Add a relaxed constraint: must be a URL if not NULL
ALTER TABLE profile
ADD CONSTRAINT valid_video_call_url CHECK (
  video_call_url IS NULL OR video_call_url ~ '^https?://'
);
