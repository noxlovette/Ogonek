-- Add migration script here
-- RFC 5545 (iCalendar) standards
CREATE TABLE calendars (
    id VARCHAR(21) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    colour VARCHAR(7) NOT NULL DEFAULT '#df7055', -- Brand colour
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC+3', -- Moscow
    owner_id VARCHAR(21) NOT NULL REFERENCES "user"(id),

    
    caldav_url VARCHAR(500), -- CalDAV collection URL
    sync_token VARCHAR(255), -- For CalDAV sync
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraint: one calendar per user (can drop later for multical support)
    CONSTRAINT unique_user_calendar UNIQUE (owner_id)
);


CREATE TABLE calendar_events (
    -- Primary identification
    id VARCHAR(21) PRIMARY KEY,
    uid VARCHAR(255) NOT NULL UNIQUE, -- iCalendar UID (globally unique)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Calendar association
    calendar_id VARCHAR(21) NOT NULL REFERENCES calendars(id) ON DELETE CASCADE,
    
    -- Basic event information
    summary VARCHAR(255) NOT NULL, -- Event title
    description TEXT,
    location TEXT,
    url TEXT, -- Associated URL
    
    -- Temporal information
    dtstart TIMESTAMPTZ NOT NULL, -- Start date/time
    dtend TIMESTAMPTZ, -- End date/time (null for all-day events)
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    timezone VARCHAR(50), -- IANA timezone identifier
    
    -- Recurrence
    rrule TEXT, -- RFC 5545 RRULE string
    rdate TEXT[], -- Additional recurrence dates
    exdate TEXT[], -- Exception dates
    recurrence_id TIMESTAMPTZ, -- For recurring event instances. Contains the DateTime of this instance
    
    -- Status and classification
    status VARCHAR(20) NOT NULL DEFAULT 'confirmed' CHECK (status IN ('tentative', 'confirmed', 'cancelled')),
    class VARCHAR(20) NOT NULL DEFAULT 'public' CHECK (class IN ('public', 'private', 'confidential')),
    transp VARCHAR(20) NOT NULL DEFAULT 'opaque' CHECK (transp IN ('opaque', 'transparent')),
    
    -- Priority and categorization
    priority INTEGER CHECK (priority BETWEEN 0 AND 9),
    categories TEXT[], -- Array of category strings
    
    -- Organiser and attendee references
    organiser_email VARCHAR(255),
    organiser_name VARCHAR(255),
    
    -- Metadata
    sequence INTEGER NOT NULL DEFAULT 0, -- Version control for updates
    dtstamp TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- Last modification timestamp
    
    -- CalDAV/WebDAV support
    etag VARCHAR(64) NOT NULL DEFAULT encode(sha256(random()::text::bytea), 'hex'),
    
    -- Soft delete support
    deleted_at TIMESTAMPTZ
    
);

CREATE TABLE event_attendees (
    id VARCHAR(21) PRIMARY KEY,
    user_id VARCHAR(21) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    event_id VARCHAR(21) NOT NULL REFERENCES calendar_events(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    role VARCHAR(20) NOT NULL DEFAULT 'req-participant' CHECK (role IN ('chair', 'req-participant', 'opt-participant', 'non-participant')),
    status VARCHAR(20) NOT NULL DEFAULT 'needs-action' CHECK (status IN ('needs-action', 'accepted', 'declined', 'tentative', 'delegated')),
    rsvp BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(event_id, email)
);

CREATE TABLE event_alarms (
    id VARCHAR(21) PRIMARY KEY,
    event_id VARCHAR(21) NOT NULL REFERENCES calendar_events(id) ON DELETE CASCADE,
    trigger_offset INTERVAL, -- e.g., '-PT15M' for 15 minutes before
    trigger_datetime TIMESTAMPTZ, -- Absolute trigger time
    action VARCHAR(20) DEFAULT 'display' CHECK (action IN ('audio', 'display', 'email', 'procedure')),
    description TEXT,
    summary VARCHAR(255),
    attendee_email TEXT[], -- For email alarms
    attendee_telegram_id TEXT[], -- For TG notifications
    attendee_apns TEXT[], -- Apple Notifications
    repeat_count INTEGER NOT NULL DEFAULT 0,
    repeat_interval INTERVAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_events_calendar_id ON calendar_events(calendar_id);
CREATE INDEX idx_events_dtstart ON calendar_events(dtstart);
CREATE INDEX idx_events_dtend ON calendar_events(dtend);
CREATE INDEX idx_events_uid ON calendar_events(uid);
CREATE INDEX idx_events_recurrence ON calendar_events(recurrence_id) WHERE recurrence_id IS NOT NULL;
CREATE INDEX idx_events_deleted ON calendar_events(deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX idx_events_time_range ON calendar_events(dtstart, dtend) WHERE deleted_at IS NULL;
CREATE INDEX idx_calendars_owner_id ON calendars(owner_id);

CREATE INDEX idx_attendees_event_id ON event_attendees(event_id);
CREATE INDEX idx_attendees_email ON event_attendees(email);

CREATE INDEX idx_alarms_event_id ON event_alarms(event_id);
CREATE INDEX idx_alarms_trigger ON event_alarms(trigger_datetime) WHERE trigger_datetime IS NOT NULL;

CREATE TRIGGER update_calendar_events_updated_at 
    BEFORE UPDATE ON calendar_events 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_event_attendees_updated_at 
    BEFORE UPDATE ON event_attendees 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_calendars_updated_at 
    BEFORE UPDATE ON calendars 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();