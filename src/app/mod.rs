use rocket;

pub mod guards;
pub mod pool;
mod slack;

pub fn start() {
  rocket::ignite()
    .manage(pool::init_pool())
    .mount("/slack", routes![slack::post])
    .launch();
}
