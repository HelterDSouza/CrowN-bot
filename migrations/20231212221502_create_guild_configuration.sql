-- Add migration script here
CREATE TABLE IF NOT EXISTS guild_configurations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_name TEXT NOT NULL,
    guild_id TEXT NOT NULL UNIQUE,
    roll_channel_id TEXT NULL UNIQUE,
    prefix TEXT DEFAULT '$' NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL
);