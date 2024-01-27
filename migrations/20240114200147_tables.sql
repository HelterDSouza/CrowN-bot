CREATE TABLE Accounts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  discord_id INT NOT NULL UNIQUE,
  username VARCHAR(255) NOT NULL,
  discriminator VARCHAR(255) NOT NULL,
  global_name VARCHAR(255)
);

CREATE TABLE GuildConfigurations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  guild_id VARCHAR(255) NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL,
  prefix VARCHAR(255) NOT NULL DEFAULT '$',
  is_active BOOLEAN NOT NULL DEFAULT true,
  roll_channel VARCHAR(255)
);

CREATE TABLE CustomImages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  image_url VARCHAR(255) NOT NULL UNIQUE,
  character_id INT NOT NULL,
  added_by INT NULL,
  is_nsfw BOOLEAN NOT NULL DEFAULT false,
  is_private BOOLEAN NOT NULL DEFAULT false,
  FOREIGN KEY (character_id) REFERENCES Characters (id),
  FOREIGN KEY (added_by) REFERENCES Accounts (id)
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
