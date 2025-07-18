-- Add migration script here

CREATE TABLE user_preferences (
    user_id VARCHAR(21) PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
    auto_subscribe BOOLEAN NOT NULL DEFAULT true,
    email_notifications BOOLEAN NOT NULL DEFAULT true,
    push_notifications BOOLEAN NOT NULL DEFAULT true,
    theme TEXT NOT NULL DEFAULT 'system' CHECK (theme IN ('light', 'dark', 'system')),
    language TEXT NOT NULL DEFAULT 'en' CHECK (language IN ('en', 'ru', 'fr', 'de', 'it'))
);

-- Create index for faster lookups
CREATE INDEX idx_user_preferences_user_id ON user_preferences(user_id);