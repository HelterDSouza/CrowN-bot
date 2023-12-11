-- Add migration script here
CREATE TABLE IF NOT EXISTS "series" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS "characters" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" TEXT UNIQUE NOT NULL,
    "series_id" INTEGER REFERENCES series(id),
    "image" TEXT UNIQUE NOT NULL
);