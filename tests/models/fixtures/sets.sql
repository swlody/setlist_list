INSERT INTO
    sets (
        created_at,
        updated_at,
        id,
        creator_id,
        dj_names,
        venue,
        city,
        event_name,
        start_time,
        duration_seconds
    )
VALUES (
        '2022-06-02T11:00:00.000',
        '2022-06-02T11:20:00.000',
        '33333333-3333-3333-3333-333333333333',
        '11111111-1111-1111-1111-111111111111',
        ARRAY [
            'Jeff Mills',
            'Frankie Knuckles'
        ],
        'Marble Bar',
        'Detroit',
        'Movement Festival',
        '2022-05-27T11:00',
        '3600'
    )