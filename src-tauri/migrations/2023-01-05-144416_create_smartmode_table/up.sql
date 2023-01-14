-- Your SQL goes here
CREATE TABLE
  smartmode (
    id INTEGER NOT NULL PRIMARY KEY,
    enabled BOOLEAN NOT NULL DEFAULT 0,
    format VARCHAR NULL,
    quality VARCHAR NULL,
    directory VARCHAR NULL
  );

INSERT INTO smartmode DEFAULT VALUES;