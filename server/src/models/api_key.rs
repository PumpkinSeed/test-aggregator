use std::ops::Add;

use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use postgres::{Error, Row};
use serde::{Deserialize, Serialize};
use simplestore::F;
use simplestore::Store;
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
        let p = serde_json::to_string(&data).unwrap();
        let ok = Store::new().put(TABLE_NAME, Uuid::new_v4().to_simple().to_string().as_str(), p);
        if ok {
            Ok(())
        } else {
            // TODO
            Ok(())
        }
    }

    pub fn update(&self, id: String) -> Result<(), Error> {
        let mut data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Ok(()) // TODOreturn Err(err),
        };

        let p = serde_json::to_string(&data).unwrap();
        let ok = Store::new().put(TABLE_NAME, id.as_str(), p);
        if ok {
            Ok(())
        } else {
            // TODO
            Ok(())
        }
    }

    pub fn get_all() -> Result<Vec<ApiKey>, Error> {
        let mut result: Vec<ApiKey> = Vec::new();

        match Store::new().fetch(TABLE_NAME) {
            Ok(val) => {
                for (id, val) in val.iter() {
                    println!("Calling {}: {}", id, val);
                    result.push(serde_json::from_str(val).unwrap())
                }
                Ok(result)
            }
            Err(e) => Ok(result) // TODO
        }
    }

    pub fn get(id: String) -> Result<ApiKey, Error> {
        match Store::new().get(TABLE_NAME, id.as_str()) {
            Ok(val) => {
                Ok(serde_json::from_str(val.as_str()).unwrap())
            }
            Err(e) => Ok(ApiKey{
                id: None,
                key: None,
                expires_at: None,
                created_at: None
            }) // TODO
        }
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

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use chrono::{NaiveDate, NaiveDateTime, Utc};

    use simplestore::F;
    use simplestore::Store;
    use crate::models::api_key::{ApiKey, TABLE_NAME};

    #[test]
    fn test_put_get() -> Result<(), serde_json::Error> {
        let s = Store { store_dir: "./simplestore" };
        let api_key = ApiKey {
            id: Some(1),
            key: Some("123key".to_string()),
            expires_at: Some(Utc::now().naive_local()),
            created_at: Some(Utc::now().naive_local()),
        };
        let start = Instant::now();
        s.put(TABLE_NAME, "12345", serde_json::to_string(&api_key).unwrap());

        let duration = start.elapsed();

        println!("Time elapsed in s.put() is: {:?}", duration);

        let start = Instant::now();
        match s.get(TABLE_NAME, "12345") {
            Ok(data) => {
                let res: ApiKey = serde_json::from_str(data.as_str())?;
                println!("{:?}",api_key);
            },
            Err(err) => println!("{}", err),
        }
        println!("Time elapsed in s.get() is: {:?}", duration);
        Ok(())
    }

    #[test]
    fn test_update_api_key() -> Result<(), serde_json::Error> {
        let s = Store { store_dir: "./simplestore" };
        let api_key = ApiKey {
            id: Some(1),
            key: Some("123key".to_string()),
            expires_at: Some(Utc::now().naive_local()),
            created_at: Some(Utc::now().naive_local()),
        };
        s.put(TABLE_NAME, "999", serde_json::to_string(&api_key).unwrap());

        let api_key = ApiKey {
            id: Some(1),
            key: Some("ASD".to_string()),
            expires_at: Some(Utc::now().naive_local()),
            created_at: Some(Utc::now().naive_local()),
        };
        s.put(TABLE_NAME, "999", serde_json::to_string(&api_key).unwrap());

        match s.get(TABLE_NAME, "999") {
            Ok(data) => {
                let res: ApiKey = serde_json::from_str(data.as_str())?;
                println!("{:?}",api_key);
            },
            Err(err) => println!("{}", err),
        }
        Ok(())
    }
}
