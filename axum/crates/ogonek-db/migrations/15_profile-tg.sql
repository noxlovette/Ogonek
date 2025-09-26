-- Add migration script here
BEGIN;

ALTER TABLE profile
DROP COLUMN bio,
DROP COLUMN quizlet_url,
DROP COLUMN timezone;

ALTER TABLE profile
ADD COLUMN telegram_id VARCHAR(20);

COMMIT;
