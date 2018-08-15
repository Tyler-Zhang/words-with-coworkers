-- This file should undo anything in `up.sql`
ALTER TABLE games ADD COLUMN board_width integer;
ALTER TABLE games ADD COLUMN board_height integer;
