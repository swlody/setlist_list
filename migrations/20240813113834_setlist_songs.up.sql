CREATE TABLE IF NOT EXISTS setlist_songs (
    setlist_id uuid NOT NULL,
    track_title text NOT NULL,
    track_artist text NOT NULL,
    track_number integer,
    track_start_time_offset_seconds integer,
    track_duration_seconds integer,
    CONSTRAINT fk_setlist FOREIGN KEY (setlist_id) REFERENCES sets (id)
);