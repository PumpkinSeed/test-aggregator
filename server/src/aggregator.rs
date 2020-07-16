use crate::store::execute;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[post("/simulationResult", data = "<result>")]
pub fn init_simulation(result: Json<SimulationResult>) -> &'static str {
    match result.insert() {
        Ok(_) => "success",
        Err(err) => "error",
    }
    // "Hello"
}

static TABLE_NAME: &'static str = "results";

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
        let res = execute(
            &insert_query[..],
            &[
                &data.id,
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

    pub fn get() {}
}

#[cfg(test)]
mod tests {
    use crate::aggregator;
    use std::time::Instant;

    #[test]
    fn storage_put() {
        let res = aggregator::SimulationResult {
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
