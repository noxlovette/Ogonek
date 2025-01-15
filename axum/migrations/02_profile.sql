CREATE TABLE profile (
    user_id VARCHAR(21) PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
    quizlet_url VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(255),
    timezone VARCHAR(50),
    CONSTRAINT valid_quizlet_url CHECK (quizlet_url ~ '^https?:\/\/quizlet\.com\/.*$')
);