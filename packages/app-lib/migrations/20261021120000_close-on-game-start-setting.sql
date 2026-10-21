ALTER TABLE settings
ADD COLUMN close_on_game_start INTEGER NOT NULL DEFAULT FALSE CHECK (close_on_game_start IN (0, 1));
