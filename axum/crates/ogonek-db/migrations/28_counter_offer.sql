-- Add migration script here
CREATE TYPE proposal_status AS ENUM ('pending', 'accepted', 'declined');

CREATE TABLE event_counter_proposals (
    id VARCHAR(21) PRIMARY KEY,
    event_id VARCHAR(21) NOT NULL REFERENCES calendar_events(id),
    proposer VARCHAR(21) NOT NULL REFERENCES "user"(id),
    proposed_start TIMESTAMPTZ,
    proposed_end TIMESTAMPTZ,
    proposed_location TEXT,
    comment TEXT,
    status proposal_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)