ALTER TABLE tasks
ADD COLUMN visibility VARCHAR NOT NULL DEFAULT 'private' 
CHECK (visibility IN ('public', 'private', 'shared'));

UPDATE tasks
SET visibility = CASE
  WHEN assignee IS NOT NULL THEN 'shared'
  ELSE 'private'
END;

ALTER TABLE tasks
DROP COLUMN IF EXISTS file_path;

ALTER TABLE tasks
ADD CONSTRAINT tasks_visibility_assignee_consistency
CHECK (
  (visibility = 'public' AND assignee IS NULL) OR
  (visibility = 'shared' AND assignee IS NOT NULL) OR
  (visibility = 'private' AND assignee IS NULL)
);

ALTER TABLE decks
DROP CONSTRAINT IF EXISTS valid_visibility;

UPDATE decks
SET visibility = 'shared'
WHERE visibility = 'assigned';

ALTER TABLE decks
ADD CONSTRAINT valid_visibility 
CHECK (visibility IN ('private', 'shared', 'public'));

UPDATE decks
SET visibility = CASE
  WHEN assignee IS NOT NULL THEN 'shared'
  WHEN visibility = 'public' THEN 'public'  -- preserve existing public decks
  ELSE 'private'
END;

ALTER TABLE decks
ADD CONSTRAINT decks_visibility_assignee_consistency
CHECK (
  (visibility = 'public' AND assignee IS NULL) OR
  (visibility = 'shared' AND assignee IS NOT NULL) OR
  (visibility = 'private' AND assignee IS NULL)
);