-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS rchat.history;
DROP TYPE IF EXISTS rchar.content CASCADE;
DROP SCHEMA IF EXISTS rchat CASCADE;