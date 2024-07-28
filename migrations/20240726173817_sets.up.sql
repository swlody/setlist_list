-- Add up migration script here
CREATE TABLE IF NOT EXISTS sets
(
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid PRIMARY KEY,
    band_name character varying NOT NULL,
    date date NOT NULL,
    venue character varying NOT NULL,
    setlist json,
    creator_id uuid NOT NULL
)