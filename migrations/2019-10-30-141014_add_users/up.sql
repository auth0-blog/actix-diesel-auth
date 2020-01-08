-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);