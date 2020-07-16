use postgres::types::ToSql;
use postgres::{Client, NoTls};

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>;

pub fn init() -> &'static Client {
    // Connection::connect("postgresql://postgres:secretpw@localhost:5432/test", TlsMode::None).unwrap()
    &Client::connect("host=localhost port=5432 user=postgres password=secretpw dbname=test", NoTls).unwrap()
}

pub fn execute(db: &Client, query: &str, params: &[&(dyn ToSql + Sync)]) -> String {
    // let mut client = match db.clone().get() {
    //     Ok(c) => c,
    //     Err(e) => return format!("ERROR: getting database connection: {}", e),
    // };
    match db.execute(query, params) {
        Ok(_) => format!("done"),
        Err(e) => format!("ERROR: performing database operation: {}", e),
    }
}

