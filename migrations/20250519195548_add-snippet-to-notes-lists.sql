-- Add migration script here
-- migrations/20250520123456_add-snippet-to-notes-lists.sql

ALTER TABLE notes_lists
ADD COLUMN snippet TEXT NOT NULL DEFAULT '';