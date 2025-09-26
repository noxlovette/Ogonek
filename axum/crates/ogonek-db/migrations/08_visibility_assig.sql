-- Step 1: Add visibility column and assignee column
ALTER TABLE decks
ADD COLUMN visibility VARCHAR(20) NOT NULL DEFAULT 'private',
ADD COLUMN assignee VARCHAR(21) REFERENCES "user"(id) ON DELETE SET NULL,
ADD CONSTRAINT valid_visibility CHECK (visibility IN ('private', 'assigned', 'public'));

-- Step 2: Migrate data from shared to visibility
UPDATE decks 
SET visibility = CASE 
    WHEN shared = TRUE THEN 'public' 
    ELSE 'private' 
END;

-- Step 3: Drop the shared column
ALTER TABLE decks
DROP COLUMN shared;