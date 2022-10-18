-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  content VARCHAR NOT NULL,
  is_finished BOOLEAN NOT NULL
)