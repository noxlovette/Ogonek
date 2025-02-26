CREATE TABLE decks (
    id VARCHAR(21) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_by VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    shared BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cards (
    id VARCHAR(21) PRIMARY KEY,
    front TEXT NOT NULL,
    back TEXT NOT NULL,
    media_url TEXT,
    deck_id VARCHAR(21) NOT NULL REFERENCES decks(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE card_progress (
    id VARCHAR(21) PRIMARY KEY,
    user_id VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    card_id VARCHAR(21) NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    review_count INT NOT NULL DEFAULT 0, -- Number of reviews
    last_reviewed TIMESTAMP WITH TIME ZONE, -- Last review timestamp
    due_date TIMESTAMP WITH TIME ZONE, -- Next scheduled review
    ease_factor FLOAT NOT NULL DEFAULT 2.5, -- SR algorithm modifier
    interval INT NOT NULL DEFAULT 1, -- Days until next review
    CONSTRAINT unique_card_user UNIQUE (user_id, card_id)
);
