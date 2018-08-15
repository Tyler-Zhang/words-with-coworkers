#![feature(plugin, try_from, custom_derive)]
#![plugin(rocket_codegen)]

extern crate dotenv;

#[macro_use]
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

extern crate rand;

extern crate regex;

use dotenv::dotenv;

pub mod models;
pub mod schema;
pub mod services;
pub mod helpers;
pub mod operations;
pub mod lib;
mod app;

fn main() {
    dotenv().ok();
    app::start();
}
