use postgres::{Client, Error, NoTls, Row};
use postgres::types::ToSql;

pub fn execute(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error> {
    let connection_string = fetch_connection_string();
    let mut db = Client::connect(&connection_string[..], NoTls).unwrap();

    db.execute(query, params)
}

pub fn query_all(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
    let connection_string = fetch_connection_string();
    let mut db = Client::connect(&connection_string[..], NoTls).unwrap();

    db.query(query, params)
}

pub fn query_one(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error> {
    let connection_string = fetch_connection_string();
    let mut db = Client::connect(&connection_string[..], NoTls).unwrap();

    db.query_one(query, params)
}

pub fn fetch_connection_string<'a>() -> String {
    static ENV_HOST: &str = "PG_HOST";
    static ENV_PORT: &str = "PG_PORT";
    static ENV_USER: &str = "PG_USER";
    static ENV_PASSWORD: &str = "PG_PASSWORD";
    static ENV_DBNAME: &str = "PG_DBNAME";

    static DEFAULT_HOST: &str = "localhost";
    static DEFAULT_PORT: &str = "5432";
    static DEFAULT_USER: &str = "postgres";
    static DEFAULT_PASSWORD: &str = "secretpw";
    static DEFAULT_DBNAME: &str = "simulator";

    let host = std::env::var(ENV_HOST).unwrap_or_else(|_| DEFAULT_HOST.into());
    let port = std::env::var(ENV_PORT).unwrap_or_else(|_| DEFAULT_PORT.into());
    let user = std::env::var(ENV_USER).unwrap_or_else(|_| DEFAULT_USER.into());
    let password = std::env::var(ENV_PASSWORD).unwrap_or_else(|_| DEFAULT_PASSWORD.into());
    let dbname = std::env::var(ENV_DBNAME).unwrap_or_else(|_| DEFAULT_DBNAME.into());

    format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, password, dbname
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_connection_string() {
        let res = fetch_connection_string();
        assert_eq!(
            res,
            String::from(
                "host=localhost port=5432 user=postgres password=secretpw dbname=simulator"
            )
        );
    }
}
