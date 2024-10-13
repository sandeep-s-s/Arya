-- Your SQL goes here
CREATE TABLE IF NOT EXISTS  requests (
  id INTEGER NOT NULL PRIMARY KEY,
  uuid VARCHAR NOT NULL UNIQUE,
  name VARCHAR NOT NULL,
  request_data VARCHAR NOT NULL,
  sort INTEGER NOT NULL DEFAULT 0,
  status BOOLEAN NOT NULL DEFAULT 0,
  create_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  update_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  collection_id INTEGER NOT NULL REFERENCES collections(id)
);