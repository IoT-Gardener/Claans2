-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orgs
(
    org_id   INT GENERATED ALWAYS AS IDENTITY,
    org_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (org_id)
);