CREATE TABLE IF NOT EXISTS sets (
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid PRIMARY KEY,
    band_name text NOT NULL,
    date date NOT NULL,
    venue text NOT NULL,
    setlist json,
    creator_id uuid NOT NULL
)