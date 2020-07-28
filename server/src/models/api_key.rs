use std::ops::Add;

use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use postgres::{Error, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::store::{execute, query_all, query_one};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    id: Option<i32>,
    key: Option<String>,
    expires_at: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
}

static TABLE_NAME: &'static str = "api_keys";
const EXPIRATION_IN_YEARS: i32 = 500;

impl ApiKey {
    pub fn insert(&self) -> Result<(), Error> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Ok(())// TODO
        };

        let insert_query = format!("INSERT INTO {} (key,created_at,expires_at) VALUES ( $1,$2,$3);", TABLE_NAME);
        match execute(&insert_query[..], &[&data.key, &Utc::now().naive_local(), &(Utc::now()  + Duration::days((EXPIRATION_IN_YEARS * 60) as i64)).naive_local()]) { // TODO expires_at
            Ok(_num) => {
                Ok(())
            }
            Err(error) => {
                // TODO handle error properly
                Err(error)
            }
        }
    }

    pub fn update(&self, id: String) -> Result<(), Error> {
        let mut data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Ok(()) // TODOreturn Err(err),
        };

        let update_query = format!("UPDATE {} SET key = $1, expires_at = $2, created_at = $3  WHERE id='{}';", TABLE_NAME, id);
        match execute(&update_query[..], &[&data.key, &data.expires_at, &data.created_at, ]) {
            Ok(_val) => Ok(()),
            Err(error) => Err(error)
        }
    }

    pub fn get_all() -> Result<Vec<ApiKey>, Error> {
        let query_str = format!(
            "SELECT * FROM {};", TABLE_NAME);

        let res = match query_all(&query_str[..], &[]) {
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
        let query_str = format!("SELECT * FROM {} WHERE id='{}';", TABLE_NAME, id);
        let row = match query_one(&query_str[..], &[]) {
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

    pub fn delete(id: String) -> Result<(), Error> {
        let delete_query = format!(
            "DELETE FROM {} WHERE id='{}';",
            TABLE_NAME, id
        );
        match execute(&delete_query[..], &[]) {
            Ok(_val) => Ok(()),
            Err(error) => Err(error)
        }
    }

    fn prepare(&self) -> Result<&Self, String> {
        Ok(self)
    } // @TODO

    fn update_expiration(&mut self) {
        match self.expires_at {
            Some(mut val) => val = val + Duration::days((EXPIRATION_IN_YEARS * 365) as i64),
            None => ()
        }
    }
}
