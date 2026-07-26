CREATE TABLE IF NOT EXISTS users (
    id              VARCHAR(64) PRIMARY KEY,
    principal_code  VARCHAR(128) NOT NULL,
    nickname        VARCHAR(128) NOT NULL,
    avatar          VARCHAR(512) NOT NULL DEFAULT '',
    email           VARCHAR(256) NOT NULL DEFAULT '',
    status          VARCHAR(32)  NOT NULL DEFAULT 'active',
    verified_name   VARCHAR(128),
    created_at      TIMESTAMPTZ  NOT NULL,
    updated_at      TIMESTAMPTZ  NOT NULL
);
