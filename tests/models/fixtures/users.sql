INSERT INTO
    users (
        id,
        email,
        password,
        api_key,
        username,
        created_at,
        updated_at
    )
VALUES (
        '11111111-1111-1111-1111-111111111111',
        'user1@example.com',
        '$argon2id$v=19$m=19456,t=2,p=1$ETQBx4rTgNAZhSaeYZKOZg$eYTdH26CRT6nUJtacLDEboP0li6xUwUF/q5nSlQ8uuc',
        '95ec80d7-cb60-4b70-9b4b-9ef74cb88758',
        'user1',
        '2023-11-12T12:34:56.789',
        '2023-11-12T12:34:56.789'
    ),
    (
        '22222222-2222-2222-2222-222222222222',
        'user2@example.com',
        '$argon2id$v=19$m=19456,t=2,p=1$ETQBx4rTgNAZhSaeYZKOZg$eYTdH26CRT6nUJtacLDEboP0li6xUwUF/q5nSlQ8uuc',
        '153561ca-fa84-4e1b-813a-c62526d0a77e',
        'user2',
        '2023-11-12T12:34:56.789',
        '2023-11-12T12:34:56.789'
    )