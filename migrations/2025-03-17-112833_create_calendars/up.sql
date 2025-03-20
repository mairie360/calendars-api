-- Your SQL goes here
CREATE TABLE
    calendars (
        id SERIAL PRIMARY KEY,
        name VARCHAR(64) UNIQUE NOT NULL,
        description VARCHAR(256) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
    );

-- CREATE TABLE
--     events (
--         id SERIAL PRIMARY KEY,
--         calendar_id INTEGER REFERENCES calendars(id) ON DELETE CASCADE,
--         name VARCHAR(64) NOT NULL,
--         description VARCHAR(256) NOT NULL,
--         start_at TIMESTAMP NOT NULL,
--         end_at TIMESTAMP NOT NULL,
--         created_at TIMESTAMP NOT NULL DEFAULT NOW(),
--         updated_at TIMESTAMP NOT NULL DEFAULT NOW()
--     );
-- CREATE TABLE
--     users (
--         id SERIAL PRIMARY KEY,
--         username VARCHAR(64) UNIQUE NOT NULL,
--         email VARCHAR(64) UNIQUE NOT NULL,
--         password VARCHAR(64) NOT NULL,
--         photo BYTEA,
--         job VARCHAR(64),
--         status VARCHAR(64),
--     );
-- CREATE TABLE
--     user_calendars (
--         user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
--         calendar_id INTEGER REFERENCES calendars(id) ON DELETE CASCADE,
--         PRIMARY KEY (user_id, calendar_id)
--     );
-- CREATE TABLE
--     user_event (
--         event_id INTEGER REFERENCES events(id) ON DELETE CASCADE,
--         user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
--         PRIMARY KEY (event_id, user_id)
--     );