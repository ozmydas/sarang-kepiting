-- Add migration script here

CREATE TABLE notes_tags (
    id SERIAL PRIMARY KEY,
    note_code VARCHAR(128) NOT NULL REFERENCES notes_lists(code) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    created_by INT NULL REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notes_tags_note_code ON notes_tags(note_code);
CREATE INDEX idx_notes_tags_tag ON notes_tags(tag);
