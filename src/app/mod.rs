use rocket;

pub mod guards;
pub mod pool;
pub mod state;
mod slack;

pub fn start() {
  rocket::ignite()
    .manage(pool::init_pool())
    .manage(state::dictionary::ScrabbleDictionary::new(dotenv!("DICTIONARY_PATH")))
    .mount("/slack", routes![slack::post])
    .launch();
}
