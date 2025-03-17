-- Your SQL goes here
CREATE TABLE
    calendars (
        id SERIAL PRIMARY KEY,
        name VARCHAR(64) UNIQUE NOT NULL,
        description VARCHAR(256) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    );