use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::vec;

mod model {
    use rocket::config::Datetime;
    include!("../models/api_key.rs");
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    // TODO routes![init_simulation,get_simulations,get_simulation,update_simulation,delete_simulation,invalidate_simulation]
    routes!(create,update,get_all,get_one,delete)
}

#[post("/apiKey", format = "application/json", data = "<api_key>")]
pub fn create(api_key: Json<model::ApiKey>) -> status::Accepted<String> {
    match api_key.insert() {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}

#[put("/apiKey/<id>", format = "application/json", data = "<api_key>")]
pub fn update(id: String, api_key: Json<model::ApiKey>) -> status::Accepted<String> {
    match api_key.update(id) {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}

#[get("/apiKey", format = "application/json")]
pub fn get_all() -> Result<Json<Vec<model::ApiKey>>, status::NotFound<String>> {
    match model::ApiKey::get_all() {
        Ok(smt) => {
            return Ok(Json(smt));
        }
        Err(err) => {
            Err(status::NotFound("Simulation results not found".to_string()))
        }
    }
}

#[get("/apiKey/<id>", format = "application/json")]
pub fn get_one(id: String) -> Result<Json<model::ApiKey>, status::NotFound<String>> {
    match model::ApiKey::get(id) {
        Ok(smt) => {
            return Ok(Json(smt));
        }
        Err(e) => {
            Err(status::NotFound("Simulation result not found".to_string()))
        }
    }
}

#[delete("/apiKey/<id>", format = "application/json")]
pub fn delete(id: String) -> status::Accepted<String> {
    match model::ApiKey::delete(id) {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}