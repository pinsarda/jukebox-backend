CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  favorite_musics INTEGER[] NOT NULL DEFAULT '{}',
  favorite_albums INTEGER[] NOT NULL DEFAULT '{}',
  favorite_artists INTEGER[] NOT NULL DEFAULT '{}',
  playlists_library INTEGER[] NOT NULL DEFAULT '{}'
);

CREATE TABLE albums (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  artists_ids INTEGER[] NOT NULL,
  description TEXT,
  fetcher TEXT,
  origin_user_id INTEGER NOT NULL,
  youtube_id TEXT,
  spotify_id TEXT,
  deezer_id TEXT,
  apple_music_id TEXT
);

CREATE TABLE musics (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  artists_ids INTEGER[] NOT NULL,
  album_id INTEGER NOT NULL,
  duration INTEGER NOT NULL,
  fetcher TEXT,
  youtube_id TEXT,
  spotify_id TEXT,
  deezer_id TEXT,
  apple_music_id TEXT,

  FOREIGN KEY (album_id) REFERENCES albums(id)
);

CREATE TABLE artists (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  youtube_id TEXT,
  spotify_id TEXT,
  deezer_id TEXT,
  apple_music_id TEXT
);

CREATE TABLE analytics (
  id SERIAL PRIMARY KEY,
  music_id INTEGER NOT NULL,
  album_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  date_played TIMESTAMP
);