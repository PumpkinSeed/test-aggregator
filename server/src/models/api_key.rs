use postgres::{Error, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod enums {
    include!("../enums.rs");
}

mod store {
    include!("../repository/simulation_result.rs");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    id: Option<i8>,
    key: Option<String>,
    expires_at: Option<String>,
    created_at: Option<String>,
}

static TABLE_NAME: &'static str = "api_keys";

impl ApiKey {
    pub fn insert(&self) -> Result<(), String> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
        // let insert_query = format!("INSERT INTO {} (id,key,created_at,expires_at) VALUES ($1, $2, $3,$4);", TABLE_NAME);
        let insert_query = format!("INSERT INTO {} (key) VALUES ($1);", TABLE_NAME);
        let res = store::execute(&insert_query[..], &[
            // &data.id,
            &data.key,
            // &data.expires_at, // TODO
            // &data.created_at, // TODO
        ]);
        if res != String::from("done") {
            return Err(res);
        }
        Ok(())
    }

    pub fn get_all() -> Result<Vec<ApiKey>, Error> {
        let query_str = format!(
            r#"
            SELECT * FROM {};
        "#,
            TABLE_NAME
        );
        let res = match store::query_all(&query_str[..], &[]) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let mut results: Vec<ApiKey> = Vec::new();
        for row in res {
            let sim_res = ApiKey {
                id: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                key: match row.try_get(1) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                expires_at: match row.try_get(2) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                created_at: match row.try_get(3) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
            };
            results.push(sim_res);
        }
        Ok(results)
    }

    pub fn get(id: String) -> Result<ApiKey, Error> {
        let query_str = format!(
            r#"SELECT * FROM {} WHERE id='{}';"#,
            TABLE_NAME, id
        );
        let row = match store::query_one(&query_str[..], &[]) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let sim_res = ApiKey {
            id: match row.try_get(0) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            key: match row.try_get(1) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            expires_at: match row.try_get(2) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            created_at: match row.try_get(3) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
        };
        return Ok(sim_res);
    }

    pub fn delete(id: String) -> Result<(), String> {
        let delete_query = format!(
            "DELETE FROM {} WHERE id='{}';",
            TABLE_NAME, id
        );
        let res = store::execute(
            &delete_query[..],
            &[],
        );
        if res != String::from("done") {
            return Err(res);
        }
        Ok(())
    }

    pub fn update(&self, id: String) -> Result<(), String> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let update_query = format!(
            "UPDATE {} SET key = $1, expires_at = $2, created_at = $3  WHERE id='{}';",
            TABLE_NAME, id);
        let res = store::execute(
            &update_query[..],
            &[
                &data.key,
                &data.expires_at,
                &data.created_at,
            ],
        );
        if res != String::from("done") {
            return Err(res);
        }
        Ok(())
    }

    fn prepare(&self) -> Result<&Self, String> {
        Ok(self)
    } // @TODO
}
