BEGIN;
  -- Drop the renamed table (bookmarks) and all its dependencies
  DROP TABLE IF EXISTS bookmarks CASCADE;
  
  -- Clean up the trigger function since it's not used anymore
  DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;
COMMIT;