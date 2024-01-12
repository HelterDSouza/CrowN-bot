-- Add migration script here


CREATE TABLE IF NOT EXISTS "custom_images" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "image_url" TEXT UNIQUE NOT NULL,
    "character_id" INTEGER REFERENCES characters(id)
);

