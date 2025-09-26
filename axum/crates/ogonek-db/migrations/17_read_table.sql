-- Add migration script here
CREATE TABLE seen_status (
    user_id VARCHAR(21) NOT NULL,
    model_type TEXT NOT NULL,
    model_id VARCHAR(21) NOT NULL,
    SEEN_AT TIMESTAMPTZ NULL,

    PRIMARY KEY (user_id, model_type, model_id)
);
