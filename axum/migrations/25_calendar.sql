-- Create ENUM types first
CREATE TYPE event_status AS ENUM ('tentative', 'confirmed', 'cancelled');
CREATE TYPE event_class AS ENUM ('public', 'private', 'confidential');
CREATE TYPE event_transp AS ENUM ('opaque', 'transparent');
CREATE TYPE attendee_role AS ENUM ('chair', 'req-participant', 'opt-participant', 'non-participant');
CREATE TYPE attendee_status AS ENUM ('needs-action', 'accepted', 'declined', 'tentative', 'delegated');
CREATE TYPE alarm_action AS ENUM ('audio', 'display', 'email', 'procedure');
CREATE TYPE sync_state AS ENUM ('active', 'syncing', 'error');
CREATE TYPE recurrence_range AS ENUM ('this-and-future'); 

-- Timezones
CREATE TABLE timezones (
    tzid VARCHAR(100) PRIMARY KEY, 
    display_name VARCHAR(200) NOT NULL,
    utc_offset_std INTEGER NOT NULL,  -- standard offset in seconds
    utc_offset_dst INTEGER,           -- DST offset in seconds (nullable)
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default timezone
INSERT INTO timezones (tzid, display_name, utc_offset_std) 
VALUES ('Europe/Moscow', 'Moscow Standard Time', 10800);

-- RFC 5545 (iCalendar) standards
CREATE TABLE calendars (
    id VARCHAR(21) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    colour VARCHAR(7) NOT NULL DEFAULT '#df7055', -- Brand colour
    timezone VARCHAR NOT NULL REFERENCES timezones(tzid) DEFAULT 'Europe/Moscow',
    owner_id VARCHAR(21) NOT NULL REFERENCES "user"(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

   -- CalDAV 
    caldav_url VARCHAR(500), -- CalDAV collection URL
    sync_token VARCHAR(255), -- For CalDAV sync
    sync_state VARCHAR(20) DEFAULT 'active' CHECK (sync_state IN ('active', 'syncing', 'error')),
    last_sync_at TIMESTAMPTZ,
    sync_error TEXT,
    -- Constraint: one calendar per user (can drop later for multical support)
    CONSTRAINT unique_user_calendar UNIQUE (owner_id)
);


CREATE TABLE calendar_events (
    -- Primary identification
    id VARCHAR(21) PRIMARY KEY,
    uid VARCHAR(255) NOT NULL, -- iCalendar UID (unique per calendar)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    master_event_id VARCHAR(21) REFERENCES calendar_events(id), -- Points to the master recurring event
    is_master_event BOOLEAN NOT NULL DEFAULT TRUE,
    
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
    dtstart_date DATE,
    dtend_date DATE,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    dtstart_tz VARCHAR NOT NULL REFERENCES timezones(tzid),
    dtend_tz VARCHAR NOT NULL REFERENCES timezones(tzid),

    CHECK (
        (all_day AND dtstart_date IS NOT NULL AND dtstart IS NULL) OR
        (NOT all_day AND dtstart IS NOT NULL AND dtstart_date IS NULL)
    ),
    
    -- Recurrence
    rrule TEXT, -- RFC 5545 RRULE string
    rdate TEXT[], -- Additional recurrence dates
    exdate TEXT[], -- Exception dates
    recurrence_id TIMESTAMPTZ, -- For recurring event instances
    recurrence_range recurrence_range, -- RANGE parameter   

    -- Status and classification (using ENUMs now)
    status event_status NOT NULL DEFAULT 'confirmed',
    class event_class NOT NULL DEFAULT 'public',
    transp event_transp NOT NULL DEFAULT 'opaque',
    
    -- Priority and categorization
    priority INTEGER CHECK (priority BETWEEN 0 AND 9),
    categories TEXT[], -- Array of category strings
    
    -- Metadata
    sequence INTEGER NOT NULL DEFAULT 0, -- Version control for updates
    dtstamp TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- Last modification timestamp
    
    -- CalDAV/WebDAV support
    etag VARCHAR(64) NOT NULL DEFAULT replace(gen_random_uuid()::text, '-', ''),
    caldav_href VARCHAR(500), -- The actual CalDAV resource URL
    content_type VARCHAR(50) DEFAULT 'text/calendar',
    
    -- Soft delete support
    deleted_at TIMESTAMPTZ,

    -- Unique constraint for uid and calendar_id
    UNIQUE(uid, calendar_id)
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

    CHECK (
        (trigger_offset IS NULL) <> (trigger_datetime IS NULL)
    )

);

-- event indices
CREATE INDEX idx_events_calendar_id ON calendar_events(calendar_id);
CREATE INDEX idx_events_dtstart ON calendar_events(dtstart);
CREATE INDEX idx_events_dtend ON calendar_events(dtend);
CREATE INDEX idx_events_uid ON calendar_events(uid);
CREATE INDEX idx_events_recurrence ON calendar_events(recurrence_id) WHERE recurrence_id IS NOT NULL;
CREATE INDEX idx_events_deleted ON calendar_events(deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX idx_events_time_range ON calendar_events(dtstart, dtend) WHERE deleted_at IS NULL;

-- calendar indices
CREATE INDEX idx_calendars_owner_id ON calendars(owner_id);
CREATE INDEX idx_events_uid_recurrence 
ON calendar_events(uid, recurrence_id);

-- attendee indices
CREATE INDEX idx_attendees_event_id ON event_attendees(event_id);
CREATE INDEX idx_attendees_email ON event_attendees(email);

-- alarm indices
CREATE INDEX idx_alarms_event_id ON event_alarms(event_id);
CREATE INDEX idx_alarms_trigger ON event_alarms(trigger_datetime) WHERE trigger_datetime IS NOT NULL;

-- Functions
CREATE OR REPLACE FUNCTION update_etag() 
RETURNS TRIGGER AS $$
BEGIN
    NEW.etag = replace(gen_random_uuid()::text, '-', '');
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- triggers
CREATE TRIGGER calendar_events_update_etag
    BEFORE UPDATE ON calendar_events
    FOR EACH ROW
    EXECUTE FUNCTION update_etag();

CREATE TRIGGER update_event_attendees_updated_at 
    BEFORE UPDATE ON event_attendees 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_calendars_updated_at 
    BEFORE UPDATE ON calendars 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();