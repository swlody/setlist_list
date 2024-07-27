-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id uuid NOT NULL,
    email character varying NOT NULL,
    password character varying NOT NULL,
    api_key character varying NOT NULL,
    username character varying NOT NULL,
    reset_token character varying,
    reset_sent_at timestamp without time zone,
    email_verification_token character varying,
    email_verification_sent_at timestamp without time zone,
    email_verified_at timestamp without time zone,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_api_key_key UNIQUE (api_key),
    CONSTRAINT users_email_key UNIQUE (email),
    CONSTRAINT users_username_key UNIQUE (username)
)