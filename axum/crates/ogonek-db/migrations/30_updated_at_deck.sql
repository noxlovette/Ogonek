-- Add migration script here
ALTER TABLE decks
ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE TRIGGER update_decks_updated_at
    BEFORE UPDATE ON decks 
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();