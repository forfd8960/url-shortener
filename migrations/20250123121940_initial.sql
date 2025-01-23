-- Add migration script here
CREATE TABLE IF NOT EXISTS short_urls (
    id bigserial PRIMARY KEY,
    origin_url VARCHAR(512) NOT NULL,
    url_uid VARCHAR(64) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS short_urls_url_uid ON short_urls(url_uid);