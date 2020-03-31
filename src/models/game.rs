use super::board::Board;
use super::player::Player;
use super::tile::TileBag;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
    pub turn: u32,
    pub tile_bag: TileBag,
    pub has_word_been_played: bool,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Board:")?;
        self.board.fmt(f)?;

        writeln!(f, "Players:")?;
        for (idx, ref player) in self.players.iter().enumerate() {
            writeln!(
                f,
                "{}: score: {}, pieces: {:?}",
                idx, player.score, player.hand
            )?;
        }

        writeln!(f, "Other:")?;
        writeln!(
            f,
            "turn: {}, has_word_been_played:{}, tile_bag: {}",
            self.turn, self.has_word_been_played, self.tile_bag
        )?;

        Ok(())
    }
}

impl Game {
    pub fn new(player_count: usize) -> Game {
        let board = Board::new();
        let mut tile_bag = TileBag::new();
        let mut players = Vec::with_capacity(player_count);

        for _ in 0..player_count {
            players.push(Player::new(&mut tile_bag));
        }

        Game {
            board: board,
            players: players,
            turn: 0,
            tile_bag: tile_bag,
            has_word_been_played: false,
        }
    }

    pub fn get_current_player(&mut self) -> &mut Player {
        let player_idx = (self.turn as usize) % self.players.len();

        &mut self.players[player_idx]
    }

    pub fn increment_turn(&mut self) {
        self.turn += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_count() {
        assert_eq!(Game::new(4).players.len(), 4);
        assert_eq!(Game::new(2).players.len(), 2);
    }

    #[test]
    fn board_size() {
        assert_eq!(Game::new(1).board.cells.len(), 225);
    }

    #[test]
    fn tile_bag_different() {
        assert_ne!(Game::new(1).tile_bag.tiles, Game::new(1).tile_bag.tiles);
    }
}
