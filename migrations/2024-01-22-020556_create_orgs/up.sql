-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orgs
(
    id   integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL
);
