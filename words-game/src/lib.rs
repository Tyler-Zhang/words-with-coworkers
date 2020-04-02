#[macro_use]
extern crate lazy_static;

mod constants;
pub mod error;
pub mod models;

pub use models::*;
pub use error::*;

pub use constants::{BOARD_SIZE, BOARD, check_dictionary};
