-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orgs
(
    org_id   integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    org_name text NOT NULL
);
