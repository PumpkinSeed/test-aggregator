use postgres::{Client, Error, NoTls, Row};
use postgres::types::ToSql;

use crate::store::fetch_connection_string;

pub fn execute(query: &str, params: &[&(dyn ToSql + Sync)]) -> String {
    let connection_string = fetch_connection_string();
    let mut db = Client::connect(&connection_string[..], NoTls).unwrap();

    match db.execute(query, params) {
        Ok(res) => {
            println!("{} rows updated", res);
            format!("done")
        },
        Err(e) => format!("ERROR: performing database operation: {}", e),
    }
}

pub fn query_all(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
    let connection_string = fetch_connection_string();
    let mut db = Client::connect(&connection_string[..], NoTls).unwrap();

    match db.query(query, params) {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}

pub fn query_one(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error> {
    let connection_string = fetch_connection_string();
    let mut db = Client::connect(&connection_string[..], NoTls).unwrap();

    match db.query_one(query, params) {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}