use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use crate::store::{execute};

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
    pub id: Option<String>,
    pub triggered_by: Option<String>,
    pub branch_name: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub commit_hash: Option<String>,
    pub status: Option<String>, 
    pub error_message: Option<String>,
    pub short_description: Option<String>,
    pub payload_data: Option<String>,
    pub payload_text: Option<String>,
    pub sequence_number: Option<i8>,
    pub invalid: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl SimulationResult {
    pub fn insert(&self) -> Result<(), String> {
        let data = match self.prepare() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
        

        let insert_query = format!(r#"
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
        "#, TABLE_NAME);
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
                &data.invalid
                // @TODO add created_at
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
}

#[cfg(test)]
mod tests {
    use crate::aggregator;
    use std::time::{Instant};

    #[test]
    fn storage_put() {
        let res = aggregator::SimulationResult{
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