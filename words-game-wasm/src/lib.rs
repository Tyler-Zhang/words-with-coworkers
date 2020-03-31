extern crate words_game;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, words-game-wasm!22");
}

#[wasm_bindgen]
pub struct Game {
    inner_game: words_game::Game,
}

#[wasm_bindgen]
pub enum Direction {
    Down,
    Right
}

#[wasm_bindgen]
impl Game {
    pub fn new(player_count: usize) -> Self {
        utils::set_panic_hook();
        Self { inner_game: words_game::Game::new(player_count) }
    }

    pub fn get_current_player_idx(&mut self) -> usize {
        self.inner_game.get_current_player_idx()
    }

    pub fn play_word(
        &mut self,
        point: &[u32],
        direction: Direction,
        word: &str
    ) {
        if let [point_x, point_y] = point {
        }
    }
}
