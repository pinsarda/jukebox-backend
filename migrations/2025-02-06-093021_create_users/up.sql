CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  userdata TEXT NOT NULL
);

CREATE TABLE albums (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  artists_ids TEXT NOT NULL,
  description TEXT,
  youtube_id TEXT
);

CREATE TABLE musics (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  artists_ids TEXT NOT NULL,
  album_id INTEGER NOT NULL,
  youtube_id TEXT
);

CREATE TABLE artists (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT
);