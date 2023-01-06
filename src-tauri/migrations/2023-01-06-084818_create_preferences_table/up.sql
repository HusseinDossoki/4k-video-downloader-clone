-- Your SQL goes here
CREATE TABLE
  preferences (
    id INTEGER NOT NULL PRIMARY KEY,
    prevent_system_sleep BOOLEAN NOT NULL DEFAULT 'f',
    create_playlist_subdirectory BOOLEAN NOT NULL DEFAULT 'f',
    numerate_playlists_files BOOLEAN NOT NULL DEFAULT 'f',
    skip_playlists_duplicates BOOLEAN NOT NULL DEFAULT 'f',
    remove_downloaded_items BOOLEAN NOT NULL DEFAULT 'f'
  );

INSERT INTO
  preferences (
    prevent_system_sleep,
    create_playlist_subdirectory,
    numerate_playlists_files,
    skip_playlists_duplicates,
    remove_downloaded_items
  )
VALUES
  (0, 0, 0, 0, 0);