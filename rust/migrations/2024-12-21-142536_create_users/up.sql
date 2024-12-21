-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- For generating UUIDs

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    password VARCHAR(128) NOT NULL,
    last_login TIMESTAMP WITH TIME ZONE,
    is_superuser BOOLEAN NOT NULL,
    username VARCHAR(150) UNIQUE NOT NULL,
    first_name VARCHAR(150) NOT NULL,
    last_name VARCHAR(150) NOT NULL,
    email VARCHAR(254) UNIQUE NOT NULL,
    is_staff BOOLEAN NOT NULL,
    is_active BOOLEAN NOT NULL,
    date_joined TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Note: The following indices are optional but can improve query performance
CREATE INDEX users_username_idx ON users(username);
CREATE INDEX users_email_idx ON users(email);