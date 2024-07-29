CREATE TABLE IF NOT EXISTS sets (
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid PRIMARY KEY,
    creator_id uuid NOT NULL,
    dj_names text [] NOT NULL,
    venue text NOT NULL,
    city text,
    event_name text,
    event_date date NOT NULL,
    doors_time timestamp,
    scheduled_start timestamp,
    actual_start timestamp,
    end_time timestamp,
    setlist json
)