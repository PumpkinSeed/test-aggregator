// proc_macro_hygiene: https://github.com/rust-lang/rust/issues/54727
// decl_macro:         https://github.com/rust-lang/rust/issues/39412
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate validator_derive;

use rocket::config::{Config, Environment};

mod aggregator;
mod store;
mod context;
mod mail;

fn main() {
    let config = match Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8012)
        .finalize()
    {
        Ok(config) => config,
        Err(e) => panic!(e),
    };

    rocket::custom(config)
        .mount("/api", routes![aggregator::init_simulation])
        .launch();

    println!("Hello, world!");
}
