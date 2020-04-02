#[macro_use]
extern crate rustler;
extern crate words_game;
extern crate serde;
extern crate serde_rustler;

use rustler::{Encoder, Env, Error, Term};
use serde::{Serialize, Deserialize};
use serde_rustler::{from_term, to_term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom right;
        atom down;
    }
}

rustler::rustler_export_nifs!(
    "Elixir.WordsGameElixir",
    [
        ("new_game", 1, new_game),
        ("play_word", 4, play_word),
    ],
    None
);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Elixir.WordsGameElixir.PlayWordResult")]
pub struct PlayWordResult {
    score: u32,
    words: Vec<String>,
}

impl From<words_game::PlayWordResult> for PlayWordResult {
    fn from(other: words_game::PlayWordResult) -> Self {
        Self { score: other.score, words: other.words}
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Elixir.WordsGameElixir.Player")]
struct Player {
    hand: String,
    score: u32,
}

impl From<words_game::Player> for Player {
    fn from(other: words_game::Player) -> Self {
        let hand = other.hand.iter()
                             .map(|x| Into::<char>::into(*x))
                             .collect();

        Self {score: other.score, hand }
    }
}

impl From<Player> for words_game::Player {
    fn from(other: Player) -> words_game::Player {
        let hand = other.hand
                    .chars()
                    .map(Into::<words_game::Tile>::into)
                    .collect();

        words_game::Player { hand, score: other.score }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Elixir.WordsGameElixir.Board")]
pub struct Board {
    pub board_dimension: u32,
    pub cells: String
}

impl From<words_game::Board> for Board {
    fn from(other: words_game::Board) -> Self {
        Self {
            board_dimension: words_game::BOARD_SIZE,
            cells: other.cells.iter().map(Into::<char>::into).collect()
        }
    }
}

impl From<Board> for words_game::Board {
    fn from(other: Board) -> Self {
        Self {
            cells: other.cells.chars()
                .map(Into::<words_game::BoardCell>::into)
                .collect()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Elixir.WordsGameElixir")]
struct Game {
    board: Board,
    players: Vec<Player>,
    turn: u32,
    tile_bag: String,
    has_word_been_played: bool
}

impl From<words_game::Game> for Game {
    fn from(other: words_game::Game) -> Game {
        Self {
            board: Into::into(other.board),
            players: other.players.into_iter().map(Into::into).collect(),
            turn: other.turn,
            tile_bag: other.tile_bag.tiles.into_iter().map(Into::<char>::into).collect(),
            has_word_been_played: other.has_word_been_played
        }
    }
}

impl From<Game> for words_game::Game {
    fn from(other: Game) -> words_game::Game {
        Self {
            board: Into::into(other.board),
            players: other.players.into_iter().map(Into::into).collect(),
            turn: other.turn,
            tile_bag: words_game::TileBag {
                tiles: other.tile_bag.chars().map(Into::<words_game::Tile>::into).collect()
            },
            has_word_been_played: other.has_word_been_played
        }
    }
}

pub fn new_game<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let player_count: u64 = args[0].decode()?;

    let game: Game = words_game::Game::new(player_count as usize).into();

    to_term(env, game).map_err(Into::into)
}

pub fn play_word<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let mut game: words_game::Game = from_term::<Game>(args[0])?.into();
    let (start_x, start_y): (i32, i32) = args[1].decode()?;
    let direction: String = args[2].decode()?;
    let word: String = args[3].decode()?;

    let play_word_result = game.play_word(
        words_game::Point::new(start_x, start_y),
        match &direction[..] {
            "right" => words_game::Direction::right(),
            "down" => words_game::Direction::down(),
            _ => return Err(Error::RaiseTerm(Box::new("Direction can only be right or down")))
        },
        &word
    ).map_err(|e| Error::RaiseTerm(Box::new(format!("{}", e))))?;


    to_term(
        env,
        (
            PlayWordResult::from(play_word_result),
            Game::from(game)
        )
    ).map_err(Into::into)
}
