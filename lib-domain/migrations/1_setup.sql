CREATE TABLE IF NOT EXISTS claw (
    id TEXT NOT NULL,
    expiry_at INTEGER NOT NULL,
    data TEXT NOT NULL,
    pem TEXT NOT NULL,
    sha256 TEXT NOT NULL,
    validity INTEGER NOT NULL,

    PRIMARY KEY (id)
);
