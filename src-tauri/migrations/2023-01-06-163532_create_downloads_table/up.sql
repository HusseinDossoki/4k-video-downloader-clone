-- Your SQL goes here
CREATE TABLE
  downloads (
    id INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NULL,
    thumbnail VARCHAR NULL,
    length_seconds INTEGER NOT NULL DEFAULT 0,
    size_in_bytes INTEGER NOT NULL DEFAULT 0,
    format VARCHAR NOT NULL DEFAULT '',
    quality VARCHAR NOT NULL DEFAULT '',
    quality_label VARCHAR NOT NULL DEFAULT '',
    progress INTEGER NOT NULL DEFAULT 0,
    time_left_sec INTEGER NOT NULL DEFAULT 0,
    approx_duration_ms INTEGER NULL,
    status VARCHAR NOT NULL DEFAULT 'new', /* new, inprogress, paused, postponed, failed, downloaded */
    url VARCHAR NOT NULL,
    directory VARCHAR NOT NULL
  );