-- Your SQL goes here
CREATE TABLE
  downloads (
    id INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NULL,
    url VARCHAR NOT NULL,
    thumbnail VARCHAR NULL,
    status VARCHAR NOT NULL DEFAULT 'new', /* new, inprogress, paused, postponed, failed, downloaded */
    directory VARCHAR NOT NULL,
    length_seconds INTEGER NOT NULL DEFAULT 0, /* time */
    size_in_bytes INTEGER NOT NULL DEFAULT 0,
    current_chunk INTEGER NULL, /* downloaded bytes */
    format VARCHAR NULL,
    quality VARCHAR NULL,
    quality_label VARCHAR NULL,
    approx_duration_ms INTEGER NULL
    -- created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );