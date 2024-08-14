INSERT INTO
    users (
        id,
        email,
        password,
        api_key,
        username,
        created_at,
        updated_at,
        email_verified_at
    )
VALUES (
        '11111111-1111-1111-1111-111111111111',
        'user@example.com',
        '$argon2id$v=19$m=19456,t=2,p=1$l/01mxJ74TkTSNPeZZ0tHQ$Wf3jBEI+/ELKHUw7h5oa7SfZkSKZVpKobtmdejz3RPI',
        '95ec80d7-cb60-4b70-9b4b-9ef74cb88758',
        'user',
        '2023-11-12T12:34:56.789',
        '2023-11-12T12:34:56.789',
        '2023-11-12T12:34:56.789'
    )
ON CONFLICT DO NOTHING