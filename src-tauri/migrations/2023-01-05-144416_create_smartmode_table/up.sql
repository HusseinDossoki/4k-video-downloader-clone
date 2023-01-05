-- Your SQL goes here
CREATE TABLE
  smartmode (
    id INTEGER NOT NULL PRIMARY KEY,
    enabled BOOLEAN NOT NULL DEFAULT 'f',
    format VARCHAR NOT NULL DEFAULT '',
    quality TEXT NOT NULL DEFAULT '',
    directory TEXT NOT NULL DEFAULT ''
  );

INSERT INTO
  smartmode (enabled, format, quality, directory)
VALUES
  (0, '', '', '');