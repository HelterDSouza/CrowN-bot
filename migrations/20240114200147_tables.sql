CREATE TABLE Accounts (
  discord_id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS GuildConfigurations (
  guild_id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  prefix VARCHAR(2) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true,
  roll_channel INTEGER
);

CREATE TABLE CustomImages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  image_url VARCHAR(255) NOT NULL UNIQUE,
  character_id INT NOT NULL,
  added_by INT NULL,
  is_nsfw BOOLEAN NOT NULL DEFAULT false,
  is_private BOOLEAN NOT NULL DEFAULT false,
  FOREIGN KEY (character_id) REFERENCES Characters (id),
  FOREIGN KEY (added_by) REFERENCES Accounts (discord_id)
);

CREATE TABLE Series (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(255) NOT NULL UNIQUE,
  is_nsfw BOOLEAN NOT NULL DEFAULT false
);


CREATE TABLE Characters (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(255) NOT NULL UNIQUE,
  image VARCHAR(255) NOT NULL,
  serie_id INT NOT NULL,
  FOREIGN KEY (serie_id) REFERENCES Series (id)
);
