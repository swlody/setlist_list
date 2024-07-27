-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.sets
(
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid,
    band_name character varying,
    date date NOT NULL,
    venue character varying,
    setlist json,
    creator_id uuid NOT NULL,
    CONSTRAINT sets_pkey PRIMARY KEY (id)
)