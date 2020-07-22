// proc_macro_hygiene: https://github.com/rust-lang/rust/issues/54727
// decl_macro:         https://github.com/rust-lang/rust/issues/39412
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate validator_derive;

use rocket::config::{Config, Environment};
mod simulation_result {
    include!("routes/simulation_result.rs");
}
mod api_key {
    include!("routes/api_key.rs");
}
mod store;

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
        .mount("/api", simulation_result::routes())
        .mount("/api",api_key::routes())
        .launch();

    println!("Hello, world!");
}
