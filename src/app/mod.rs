use rocket;
use std::env;

pub mod guards;
pub mod pool;
pub mod state;
pub mod slack;

pub fn start() {
  rocket::ignite()
    .manage(pool::init_pool())
    .manage(state::dictionary::ScrabbleDictionary::new(&env::var("DICTIONARY_PATH").unwrap()))
    .mount("/slack", routes![slack::post])
    .launch();
}
