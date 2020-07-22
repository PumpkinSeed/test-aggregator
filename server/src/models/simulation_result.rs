use postgres::{Error, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod enums {
    include!("../enums.rs");
}

// use crate::store::{execute, query_all, query_one};
mod store {
    include!("../repository/simulation_result.rs");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationResult {
    id: Option<String>,
    triggered_by: Option<String>,
    branch_name: Option<String>,
    start_timestamp: Option<i64>,
    end_timestamp: Option<i64>,
    commit_hash: Option<String>,
    status: Option<String>,
    error_message: Option<String>,
    short_description: Option<String>,
    payload_data: Option<String>,
    payload_text: Option<String>,
    sequence_number: Option<i8>,
    invalid: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

static TABLE_NAME: &'static str = "results";

impl SimulationResult {
    pub fn insert(&self) -> Result<(), String> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let insert_query = format!(
            r#"
            INSERT INTO {} (
                id,
                triggered_by,
                branch_name,
                start_timestamp,
                end_timestamp,
                commit_hash,
                status,
                error_message,
                short_description,
                payload_data,
                payload_text,
                invalid
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
        "#,
            TABLE_NAME
        );
        let res = store::execute(
            &insert_query[..],
            &[
                &Uuid::new_v4().to_string(),
                &data.triggered_by,
                &data.branch_name,
                &data.start_timestamp,
                &data.end_timestamp,
                &data.commit_hash,
                &data.status,
                &data.error_message,
                &data.short_description,
                &data.payload_data,
                &data.payload_text,
                &data.invalid, // @TODO add created_at
            ],
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
            "UPDATE {} SET triggered_by = $1, branch_name = $2, start_timestamp = $3, end_timestamp = $4, commit_hash = $5, status = $6, error_message = $7, short_description = $8, payload_data = $9, payload_text = $10, invalid = $11 WHERE id='{}';",
            TABLE_NAME, id);
        let res = store::execute(
            &update_query[..],
            &[
                &data.triggered_by,
                &data.branch_name,
                &data.start_timestamp,
                &data.end_timestamp,
                &data.commit_hash,
                &data.status,
                &data.error_message,
                &data.short_description,
                &data.payload_data,
                &data.payload_text,
                &data.invalid, // @TODO add created_at
            ],
        );
        if res != String::from("done") {
            return Err(res);
        }
        Ok(())
    }

    fn prepare(&self) -> Result<&Self, String> {
        Ok(self) // @TODO
    }

    pub fn get_all() -> Result<Vec<SimulationResult>, Error> {
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
        let mut results: Vec<SimulationResult> = Vec::new();
        for row in res {
            let sim_res = SimulationResult {
                id: match row.try_get(0) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                triggered_by: match row.try_get(1) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                branch_name: match row.try_get(2) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                start_timestamp: match row.try_get(3) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                end_timestamp: match row.try_get(4) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                commit_hash: match row.try_get(5) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                status: match row.try_get(6) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                error_message: match row.try_get(7) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                short_description: match row.try_get(8) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                payload_data: match row.try_get(9) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                payload_text: match row.try_get(10) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                sequence_number: match row.try_get(11) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                invalid: match row.try_get(12) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                created_at: match row.try_get(13) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                updated_at: match row.try_get(14) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
            };
            results.push(sim_res);
        }
        Ok(results)
    }

    pub fn get(id: String) -> Result<SimulationResult, Error> {
        let query_str = format!(
            r#"SELECT * FROM {} WHERE id='{}';"#,
            TABLE_NAME, id
        );
        let row = match store::query_one(&query_str[..], &[]) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let sim_res = SimulationResult {
            id: match row.try_get(0) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            triggered_by: match row.try_get(1) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            branch_name: match row.try_get(2) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            start_timestamp: match row.try_get(3) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            end_timestamp: match row.try_get(4) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            commit_hash: match row.try_get(5) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            status: match row.try_get(6) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            error_message: match row.try_get(7) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            short_description: match row.try_get(8) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            payload_data: match row.try_get(9) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            payload_text: match row.try_get(10) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            sequence_number: match row.try_get(11) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            invalid: match row.try_get(12) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            created_at: match row.try_get(13) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            updated_at: match row.try_get(14) {
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

    pub fn invalidate(&self, id: String) -> Result<(), String> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let update_query = format!(
            "UPDATE {} SET status = '{}' WHERE id='{}';",
            TABLE_NAME, enums::SimulationResultValidity::Invalid.as_str(), id);
        let res = store::execute(
            &update_query[..],
            &[],
        );
        if res != String::from("done") {
            return Err(res);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    // mod model {
    //     include!("../models/simulation_result");
    // }
    use crate::aggregator;

    #[test]
    fn storage_put() {
        let res = model::SimulationResult {
            id: Option::from(String::from("lalala")),
            triggered_by: Option::from(String::from("lalala")),
            branch_name: Option::from(String::from("lalala")),
            start_timestamp: Option::from(1234451231),
            end_timestamp: Option::from(1234451236),
            commit_hash: Option::from(String::from("lalala")),
            status: Option::from(String::from("lalala")),
            error_message: Option::from(String::from("lalala")),
            short_description: Option::from(String::from("lalala")),
            payload_data: Option::from(String::from("lalala")),
            payload_text: Option::from(String::from("lalala")),
            sequence_number: Option::from(5),
            invalid: Option::from(false),
            created_at: Option::from(String::from("lalala")),
            updated_at: Option::from(String::from("lalala")),
        };

        let now = Instant::now();
        match res.insert() {
            Ok(_) => println!("ok"),
            Err(err) => println!("{:?}", err),
        }

        println!("{}", now.elapsed().as_nanos());
    }
}
