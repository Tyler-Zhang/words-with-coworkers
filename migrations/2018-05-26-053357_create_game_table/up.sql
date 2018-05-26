-- Your SQL goes here
CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  board CHAR(225) NOT NULL,
  board_width INTEGER NOT NULL,
  board_height INTEGER NOT NULL,
  turn_count INTEGER NOT NULL,
  pieces VARCHAR(100) NOT NULL,
  channel_id VARCHAR(10) NOT NULL,
  player_turn_id INTEGER
);

CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL,
  pieces VARCHAR(10) NOT NULL,
  slack_id VARCHAR(10) NOT NULL,
  points INTEGER NOT NULL
);

-- Add foreign key constraint to player
ALTER TABLE players
  ADD CONSTRAINT fk_game
  FOREIGN KEY (game_id)
  REFERENCES games (id);

ALTER TABLE games
  ADD CONSTRAINT fk_player_turn
  FOREIGN KEY (player_turn_id)
  REFERENCES players (id);
