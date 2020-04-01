extern crate words_game;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game(words_game::Game);

/**
 * Necessary to redefine our own direction struct because the
 * one used in words_game stores fields which are not supported
 * by wasm-bindgen right now
 */
#[wasm_bindgen]
pub enum Direction {
    Down,
    Right
}

impl Into<words_game::Direction> for Direction {
    fn into(self) -> words_game::Direction {
        match self {
            Direction::Down => words_game::Direction::down(),
            Direction::Right => words_game::Direction::right(),
        }
    }
}

#[wasm_bindgen]
pub struct PlayWordResult {
    score: u32,
    words: Vec<String>,
}

impl From<words_game::PlayWordResult> for PlayWordResult {
    fn from(other: words_game::PlayWordResult) -> Self {
        Self { score: other.score, words: other.words}
    }
}

#[wasm_bindgen]
pub struct Player {
    pub score: u32,
    /*
     * Skip this field because it is not copy, we manually
     * define our own getters and setters
     */
    #[wasm_bindgen(skip)]
    pub hand: String,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(getter)]
    pub fn hand(&self) -> String {
        self.hand.clone()
    }
}

impl From<&words_game::Player> for Player {
    fn from(other: &words_game::Player) -> Self {
        let hand = other.hand.iter()
                             .map(|x| Into::<char>::into(*x))
                             .collect();

        Self {score: other.score, hand }
    }
}

#[wasm_bindgen]
pub struct Board {
    pub board_dimension: u32,

    #[wasm_bindgen(skip)]
    pub cells: String
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(getter)]
    pub fn cells(&self) -> String {
        self.cells.clone()
    }
}

impl From<&words_game::Board> for Board {
    fn from(other: &words_game::Board) -> Self {
        Self {
            board_dimension: words_game::BOARD_SIZE,
            cells: other.cells.iter().map(Into::<char>::into).collect()
        }
    }
}

pub type Result<T> = std::result::Result<T, JsValue>;

fn err_mapper(err: Box<words_game::Error>) -> JsValue {
    JsValue::from_str(&format!("{}", err))
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(player_count: usize) -> Self {
        utils::set_panic_hook();
        Self(words_game::Game::new(player_count))
    }

    pub fn get_current_player_idx(&mut self) -> usize {
        self.0.get_current_player_idx()
    }

    pub fn play_word(
        &mut self,
        point: &[i32],
        direction: Direction,
        word: &str
    ) -> Result<PlayWordResult> {
        if let [point_x, point_y] = *point {
            self.0.play_word(
                words_game::Point::new(point_x, point_y),
                direction.into(),
                word
            )
            .map(|par| par.into())
            .map_err(err_mapper)
        } else {
            Err("Point must be a 2 sized tuple".into())
        }
    }

    #[wasm_bindgen(getter)]
    pub fn players(&self) -> Vec<JsValue> {
        self.0.players.iter().map(|x| JsValue::from(Player::from(x))).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn board(&self) -> Board {
        Into::into(&self.0.board)
    }
}
