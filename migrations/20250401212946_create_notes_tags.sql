-- Add migration script here

CREATE TABLE notes_tags (
    id SERIAL PRIMARY KEY,
    note_id INT NOT NULL REFERENCES notes_lists(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notes_tags_note_id ON notes_tags(note_id);
CREATE INDEX idx_notes_tags_tag ON notes_tags(tag);
