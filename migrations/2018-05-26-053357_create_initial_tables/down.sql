-- This file should undo anything in `up.sql`
ALTER TABLE players DROP CONSTRAINT fk_game;
ALTER TABLE games DROP CONSTRAINT fk_player_turn;

DROP TABLE players;
DROP TABLE games;
DROP TABLE teams;
