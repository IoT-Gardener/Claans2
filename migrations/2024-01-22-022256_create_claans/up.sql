-- Your SQL goes here
CREATE TABLE IF NOT EXISTS claans
(
    claan_id   INT GENERATED ALWAYS AS IDENTITY,
    claan_name VARCHAR(255) NOT NULL,
    org_id     INT REFERENCES orgs (org_id) ON DELETE CASCADE,
    PRIMARY KEY (claan_id)
);
CREATE INDEX idx_claans_fkeys ON claans(org_id);