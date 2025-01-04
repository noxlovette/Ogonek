-- Add migration script here
ALTER TABLE "user"
    ALTER COLUMN verified SET NOT NULL,
    ALTER COLUMN joined SET NOT NULL;