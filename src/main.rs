#![feature(plugin, try_from, custom_derive)]
#![plugin(rocket_codegen)]

extern crate auto_impl;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_urlencoded;

extern crate uuid;
extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate dotenv_codegen;

extern crate rand;

pub mod models;
pub mod schema;
pub mod operations;
mod app;

fn main() {
    app::start();
}
