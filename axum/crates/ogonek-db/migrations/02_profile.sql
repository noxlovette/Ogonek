CREATE TABLE profile (
    user_id VARCHAR(21) PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
    quizlet_url VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(255),
    zoom_url VARCHAR(255),
    timezone VARCHAR(50),
    CONSTRAINT valid_quizlet_url CHECK (quizlet_url ~ '^https?:\/\/quizlet\.com\/.*$'),
    CONSTRAINT valid_zoom_url CHECK (zoom_url ~ '^https?:\/\/(?:[a-z0-9-]+\.)?zoom\.us\/j\/\d{9,11}(?:\?pwd=[a-zA-Z0-9]+)?$')
);

CREATE FUNCTION create_profile_for_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO profile (user_id)
    VALUES (NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_profile_after_user_insert
    AFTER INSERT ON "user"
    FOR EACH ROW
    EXECUTE FUNCTION create_profile_for_user();