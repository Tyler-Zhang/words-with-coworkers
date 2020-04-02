#[macro_use]
extern crate rustler;
extern crate words_game;
extern crate serde;
extern crate serde_rustler;

use std::fmt;
use rustler::{Encoder, Env, Error, Term};
use serde::{Serialize, Deserialize};
use serde_rustler::{from_term, to_term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
        atom right;
        atom down;
    }
}

rustler::rustler_export_nifs!(
    "Elixir.WordsGameElixir",
    [
        ("new_game", 1, new_game),
        ("play_word", 4, play_word),
        ("check_dictionary", 1, check_dictionary),
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

macro_rules! handle_result {
    ($result:expr, $env:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => return to_elixir_err(e, $env)
        }
    }
}

fn to_elixir_err<'a, E: fmt::Display>(e: E, env: Env<'a>) -> Result<Term<'a>, Error> {
    Ok((atoms::error(), format!("{}", e)).encode(env))
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

    let play_word_result_result = game.play_word(
        words_game::Point::new(start_x, start_y),
        match &direction[..] {
            "right" => words_game::Direction::right(),
            "down" => words_game::Direction::down(),
            _ => return Ok((atoms::error(), "Direction can only be right or down").encode(env))
        },
        &word
    );

    let play_word_result = handle_result!(play_word_result_result, env);

    Ok((
        atoms::ok(),
        to_term(env, PlayWordResult::from(play_word_result)).map_err(Into::<Error>::into)?,
        to_term(env, Game::from(game)).map_err(Into::<Error>::into)?
    ).encode(env))
}

pub fn check_dictionary<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let word: String = args[0].decode()?;

    Ok(words_game::check_dictionary(&word).encode(env))
}
