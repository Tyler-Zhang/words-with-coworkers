-- This file should undo anything in `up.sql`
ALTER TABLE games ADD COLUMN board_width integer;
ALTER TABLE games ADD COLUMN board_height integer;

UPDATE games SET board_width=15, board_height=15;
