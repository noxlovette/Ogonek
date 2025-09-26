-- Add migration script here
-- Add subscribers support to decks
ALTER TABLE decks
ADD COLUMN subscribers TEXT[] DEFAULT '{}';

-- Create a dedicated junction table for deck subscriptions for better querying
CREATE TABLE deck_subscriptions (
    deck_id VARCHAR(21) NOT NULL REFERENCES decks(id) ON DELETE CASCADE,
    user_id VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    PRIMARY KEY (deck_id, user_id)
);

-- Create index for efficient lookup
CREATE INDEX idx_deck_subscriptions_user_id ON deck_subscriptions(user_id);

CREATE OR REPLACE FUNCTION update_deck_subscribers()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Add user to subscribers array
        UPDATE decks 
        SET subscribers = array_append(subscribers, NEW.user_id)
        WHERE id = NEW.deck_id;
    ELSIF TG_OP = 'DELETE' THEN
        -- Remove user from subscribers array
        UPDATE decks 
        SET subscribers = array_remove(subscribers, OLD.user_id)
        WHERE id = OLD.deck_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for the junction table
CREATE TRIGGER update_deck_subscribers_trigger
AFTER INSERT OR DELETE ON deck_subscriptions
FOR EACH ROW EXECUTE FUNCTION update_deck_subscribers();

CREATE OR REPLACE FUNCTION create_card_progress_for_subscribers()
RETURNS TRIGGER AS $$
DECLARE
    subscriber_id VARCHAR(21);
BEGIN
    -- For each subscriber of the deck, create a card_progress entry
    FOR subscriber_id IN 
        SELECT user_id FROM deck_subscriptions WHERE deck_id = NEW.deck_id
    LOOP
        INSERT INTO card_progress (
            id, user_id, card_id, review_count, due_date, ease_factor, interval
        ) VALUES (
            nanoid(),
            subscriber_id,
            NEW.id,
            0,
            CURRENT_TIMESTAMP,
            2.5,
            1
        )
        ON CONFLICT (user_id, card_id) DO NOTHING; -- Avoid duplicates
    END LOOP;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create nanoid function for PostgreSQL (since you can't directly call nanoid from triggers)
CREATE OR REPLACE FUNCTION nanoid(size INT DEFAULT 21)
RETURNS TEXT AS $$
DECLARE
    id TEXT := '';
    i INT := 0;
    alphabet TEXT := '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    alphabet_length INT := length(alphabet);
BEGIN
    WHILE i < size LOOP
        id := id || substr(alphabet, 1 + floor(random() * alphabet_length)::INT, 1);
        i := i + 1;
    END LOOP;
    RETURN id;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for cards
CREATE TRIGGER create_card_progress_for_subscribers_trigger
AFTER INSERT ON cards
FOR EACH ROW EXECUTE FUNCTION create_card_progress_for_subscribers();