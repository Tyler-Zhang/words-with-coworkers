use rocket;

mod slack;

pub fn start() {
  rocket::ignite()
    .mount("/slack", routes![slack::post])
    .launch();
}
