-- Add migration script here
CREATE TABLE IF NOT EXISTS guild_configurations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    prefix TEXT DEFAULT '$' NOT NULL,
    is_active BOOLEAN DEFAULT 1
);