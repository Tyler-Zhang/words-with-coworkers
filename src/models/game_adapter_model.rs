// This struct links together the DB models
// with the scrabble Game object
use std::collections::HashSet;
use ::lib::scrabble;
use ::models::{Game, Player};

pub struct GameAdapter {
    pub scrabble_game: scrabble::Game,

    pub db_game: Game,
    pub db_players: Vec<Player>
}

impl GameAdapter {
    pub fn new(scrabble_game: scrabble::Game, db_game: Game, db_players: Vec<Player>) -> GameAdapter{
        GameAdapter {
            scrabble_game,
            db_game,
            db_players
        }
    }

    pub fn hydrate(db_game: Game, db_players: Vec<Player>) -> GameAdapter {
        let scrabble_game = scrabble::Game::hydrate(
            scrabble::Board::from(&db_game.board[..]),
            db_players.iter().map(|db_player| scrabble::Player::hydrate(&db_player.pieces, db_player.points)).collect(),
            db_game.turn_count,
            &db_game.pieces
        );

        GameAdapter {
            scrabble_game,
            db_game,
            db_players
        }
    }

    pub fn play_move(&mut self, word: &str, start: (i32, i32), direction_down: bool, dict: &HashSet<String>) ->
        Result<scrabble::models::Action, String> {

            let result_from_play = self.scrabble_game.play(word, start, direction_down, dict);
            self.sync_with_scrabble_game();
            result_from_play
    }

    pub fn get_player_on_turn(&self) -> &Player {
        let index = self.scrabble_game.get_player_turn_index();

        &self.db_players[index as usize]
    }

    pub fn get_player_on_turn_mut(&mut self) -> &mut Player {
        let index = self.scrabble_game.get_player_turn_index();

        &mut self.db_players[index as usize]
    }

    pub fn get_player_by_user_id_mut(&mut self, slack_id: &str) -> Option<&mut Player> {
        for player in self.db_players.iter_mut() {
            if player.slack_id == slack_id {
                return Some(player);
            }
        }

        None
    }

    pub fn sync_with_scrabble_game(&mut self) {
        // Sync game model
        self.db_game.board = self.scrabble_game.board.to_string();
        self.db_game.turn_count = self.scrabble_game.turn_count;
        let player_turn = self.scrabble_game.get_player_turn_index();
        self.db_game.player_turn_id = Some(self.db_players[player_turn as usize].id);
        self.db_game.pieces = self.scrabble_game.pieces.clone();

        let scrabble_players = &self.scrabble_game.players;

        // Sync player models
        for (index, db_player) in self.db_players.iter_mut().enumerate() {
            db_player.pieces = scrabble_players[index].pieces.clone();
            db_player.points = scrabble_players[index].score;
        }
    }
}
