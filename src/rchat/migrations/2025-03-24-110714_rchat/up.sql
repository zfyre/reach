-- Your SQL goes here
CREATE SCHEMA IF NOT EXISTS rchat;

CREATE TYPE rchat.content AS (
    "user" TEXT,
    "system" TEXT
);

CREATE TABLE rchat.history (
    "session_id" INTEGER NOT NULL,
    "message_id" SERIAL NOT NULL,
    "level" INTEGER NOT NULL,
    "content" rchat.content NOT NULL,
    "tags" TEXT[] NOT NULL,
    PRIMARY KEY ("session_id", "message_id", "level")
);


