use postgres::{Client};
use std::sync::{Mutex};
use crate::store;

pub struct Context {
    pub database: &'static Client,
}

impl Context {
    pub fn new_safe() -> Mutex<Context> {
        Mutex::new(Context{
            database: store::init(),
        })
    }
}
