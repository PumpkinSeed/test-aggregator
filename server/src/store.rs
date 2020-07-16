use postgres::types::ToSql;
use postgres::{Client, NoTls};

pub fn execute(query: &str, params: &[&(dyn ToSql + Sync)]) -> String {
    let mut db = Client::connect("host=localhost port=5432 user=postgres password=secretpw dbname=simulator", NoTls).unwrap();

    match db.execute(query, params) {
        Ok(_) => format!("done"),
        Err(e) => format!("ERROR: performing database operation: {}", e),
    }
}

fn fetch_connection_string<'a>() -> &'a str {
    ""
}
