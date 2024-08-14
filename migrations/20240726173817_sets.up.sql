CREATE TABLE IF NOT EXISTS sets (
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid PRIMARY KEY,
    creator_id uuid NOT NULL,
    dj_names text [] NOT NULL,
    venue text NOT NULL,
    city text,
    event_name text,
    start_time timestamp NOT NULL,
    duration_seconds integer
)