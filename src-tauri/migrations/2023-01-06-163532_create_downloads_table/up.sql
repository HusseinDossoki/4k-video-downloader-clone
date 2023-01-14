-- Your SQL goes here
CREATE TABLE
  downloads (
    id INTEGER NOT NULL PRIMARY KEY,

    -- Required when inserting
    url VARCHAR NOT NULL,
    directory VARCHAR NOT NULL,
    format VARCHAR NOT NULL,
    quality VARCHAR NOT NULL,
    quality_label VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'queued', -- queued, parsing, parsed, downloading, paused, postponed, failed, downloaded
    -- created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    -- Will be updated later 
    title VARCHAR NULL,
    file_name VARCHAR NULL,
    thumbnail VARCHAR NULL,
    length_seconds INTEGER NULL, -- time
    size_in_bytes INTEGER NULL,
    current_chunk INTEGER NULL, -- downloaded bytes
    approx_duration_ms INTEGER NULL
  );