CREATE TABLE IF NOT EXISTS users (
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid NOT NULL PRIMARY KEY,
    email text NOT NULL UNIQUE,
    password text NOT NULL,
    api_key uuid NOT NULL UNIQUE,
    username text NOT NULL UNIQUE,
    reset_token uuid,
    reset_sent_at timestamp,
    email_verification_token uuid,
    email_verification_sent_at timestamp,
    email_verified_at timestamp
)