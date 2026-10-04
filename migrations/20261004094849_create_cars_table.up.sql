-- Add up migration script here
CREATE TABLE IF NOT EXISTS cars (
    id UUID PRIMARY KEY NOT NULL,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    spotted INTEGER NOT NULL,
    created_at TIMESTAMPTZ NULL DEFAULT NOW()
);
