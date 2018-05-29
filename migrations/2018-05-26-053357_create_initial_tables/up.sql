-- Your SQL goes here
CREATE TABLE teams (
  id VARCHAR(10) PRIMARY KEY NOT NULL,
  team_domain VARCHAR(10) NOT NULL,
  access_token VARCHAR(80) NOT NULL,
  bot_user_access_token VARCHAR(55) NOT NULL
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  board CHAR(225) NOT NULL,
  board_width INTEGER NOT NULL,
  board_height INTEGER NOT NULL,
  turn_count INTEGER NOT NULL,
  pieces VARCHAR(100) NOT NULL,
  channel_id VARCHAR(10) NOT NULL,
  player_turn_id INTEGER,
  team_id VARCHAR(10) NOT NULL
);

CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL,
  pieces VARCHAR(10) NOT NULL,
  slack_id VARCHAR(10) NOT NULL,
  points INTEGER NOT NULL,
  team_id VARCHAR(10) NOT NULL
);

-- Add foreign key constraints
ALTER TABLE players
  ADD CONSTRAINT fk_game
  FOREIGN KEY (game_id)
  REFERENCES games (id);

ALTER TABLE games
  ADD CONSTRAINT fk_player_turn
  FOREIGN KEY (player_turn_id)
  REFERENCES players (id);

-- ALTER TABLE games
--   ADD CONSTRAINT fk_team
--   FOREIGN KEY (team_id)
--   REFERENCES teams (id);

-- ALTER TABLE players
--   ADD CONSTRAINT fk_team
--   FOREIGN KEY (team_id)
--   REFERENCES teams (id);
