-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid NOT NULL PRIMARY KEY,
    email character varying NOT NULL UNIQUE,
    password character varying NOT NULL,
    api_key character varying NOT NULL UNIQUE,
    username character varying NOT NULL UNIQUE,
    reset_token character varying,
    reset_sent_at timestamp without time zone,
    email_verification_token character varying,
    email_verification_sent_at timestamp without time zone,
    email_verified_at timestamp without time zone
)