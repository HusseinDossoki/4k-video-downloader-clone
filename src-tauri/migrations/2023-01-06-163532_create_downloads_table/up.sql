-- Your SQL goes here
CREATE TABLE
  downloads (
    id INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL DEFAULT '',
    short_description TEXT NULL,
    length_seconds INTEGER NOT NULL DEFAULT 0,
    size VARCHAR NOT NULL DEFAULT '',
    format VARCHAR NOT NULL DEFAULT '',
    quality VARCHAR NOT NULL DEFAULT '',
    progress INTEGER NOT NULL DEFAULT 0,
    status VARCHAR NOT NULL DEFAULT 'new', /* new, inprogress, paused, postponed, failed, downloaded */
    url VARCHAR NOT NULL,
    directory VARCHAR NOT NULL
  );