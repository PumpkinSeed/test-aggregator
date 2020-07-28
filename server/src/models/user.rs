use postgres::{Error, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use crate::repository::store::{execute, query_all, query_one};

mod enums {
    include!("../enums.rs");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Option<String>,
    password_hash: Option<String>,
    //TODO generate password hash
    email: Option<String>,
    git_user: Option<String>,
    role: Option<String>,
    // TODO
    notification: Option<bool>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

static TABLE_NAME: &'static str = "users";

impl User {
    pub fn insert(&self) -> Result<(), Error> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Ok(()) // TODO return Err(err),
        };

        let insert_query = format!("INSERT INTO {} (id,password_hash, email, git_user, notification, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7);", TABLE_NAME);
        match execute(
            &insert_query[..],
            &[
                &Uuid::new_v4().to_string(),
                &data.password_hash,
                &data.email,
                &data.git_user,
                &data.notification,
                &Utc::now().naive_local(),
                &Utc::now().naive_local(),
            ],
        ) {
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
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Ok(()) // TODO  return Err(err),
        };

        let update_query = format!("UPDATE {} SET password_hash = $1, email = $2, git_user = $3, role = $4, notification = $5, updated_at = $6 WHERE id='{}';", TABLE_NAME, id);
        match execute(
            &update_query[..],
            &[
                &data.password_hash,
                &data.email,
                &data.git_user,
                &data.role,
                &data.notification,
                &Utc::now().naive_local(),
            ],
        ) {
            Ok(_num) => {
                Ok(())
            }
            Err(error) => {
                // TODO handle error properly
                Err(error)
            }
        }
    }

    pub fn get_all() -> Result<Vec<User>, Error> {
        let query_str = format!("SELECT * FROM {};", TABLE_NAME);
        let res = match query_all(&query_str[..], &[]) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let mut results: Vec<User> = Vec::new();
        for row in res {
            let user = User {
                id: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                password_hash: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                email: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                git_user: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                role: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                notification: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                created_at: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                updated_at: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                // created_at: match row.try_get(13) {
                //     Ok(val) => Some(val),
                //     Err(_) => None,
                // },
                // updated_at: match row.try_get(14) {
                //     Ok(val) => Some(val),
                //     Err(_) => None,
                // },
            };
            results.push(user);
        }
        Ok(results)
    }

    pub fn get(id: String) -> Result<User, Error> {
        let query_str = format!("SELECT * FROM {} WHERE id='{}';", TABLE_NAME, id);
        let row = match query_one(&query_str[..], &[]) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let user = User {
            id: match row.try_get(0) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            password_hash: match row.try_get(1) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            email: match row.try_get(2) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            git_user: match row.try_get(3) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            role: match row.try_get(4) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            notification: match row.try_get(5) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            created_at: match row.try_get(6) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            updated_at: match row.try_get(7) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            // created_at: match row.try_get(13) {
            //     Ok(val) => Some(val),
            //     Err(_) => None,
            // },
            // updated_at: match row.try_get(14) {
            //     Ok(val) => Some(val),
            //     Err(_) => None,
            // },
        };
        return Ok(user);
    }

    pub fn delete(id: String) -> Result<(), Error> {
        let delete_query = format!("DELETE FROM {} WHERE id='{}';", TABLE_NAME, id);
        match execute(
            &delete_query[..],
            &[],
        ) {
            Ok(_num) => {
                Ok(())
            }
            Err(error) => {
                // TODO handle error properly
                Err(error)
            }
        }
    }

    fn prepare(&self) -> Result<&Self, String> {
        Ok(self) // @TODO
    }

    // pub fn get_by_git_user(git_user: String) -> Result<User, Error> {
    //     let query_str = format!(
    //         "SELECT * FROM {} WHERE git_user='{}';",
    //         TABLE_NAME, git_user
    //     );
    //     let row = match query_one(&query_str[..], &[]) {
    //         Ok(val) => val,
    //         Err(e) => return Err(e),
    //     };
    //     let user = User {
    //         id: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         password_hash: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         email: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         git_user: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         role: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         notification: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         created_at: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         updated_at: match row.try_get(0) {
    //             Ok(val) => Some(val),
    //             Err(_) => None,
    //         },
    //         // created_at: match row.try_get(13) {
    //         //     Ok(val) => Some(val),
    //         //     Err(_) => None,
    //         // },
    //         // updated_at: match row.try_get(14) {
    //         //     Ok(val) => Some(val),
    //         //     Err(_) => None,
    //         // },
    //     };
    //     return Ok(user);
    // }
}
