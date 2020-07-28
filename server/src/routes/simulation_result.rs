use std::vec;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::models::{response::ApiResponse, simulation_result as model};

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![init_simulation,get_simulations,get_simulation,update_simulation,delete_simulation,invalidate_simulation]
}

#[post("/simulationResult", format = "application/json", data = "<result>")]
pub fn init_simulation(result: Json<model::SimulationResult>) -> ApiResponse {
    match result.insert() {
        Ok(()) => {
            ApiResponse {
                json: json!(""),
                status: Status::Created,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!(format!("{}",err)),
                status: Status::InternalServerError,
            }
        }
    }
}

#[put("/simulationResult/<id>", format = "application/json", data = "<result>")]
pub fn update_simulation(id: String, result: Json<model::SimulationResult>) -> ApiResponse {
    match result.update(id) {
        Ok(val) => {
            ApiResponse {
                json: json!(val),
                status: Status::Ok,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!(format!("{}",err)),
                status: Status::InternalServerError,
            }
        }
    }
}

#[get("/simulationResult", format = "application/json")]
pub fn get_simulations() -> ApiResponse {
    match model::SimulationResult::get_all() {
        Ok(val) => {
            ApiResponse {
                json: json!(val),
                status: Status::Ok,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!("api key not found"),
                status: Status::InternalServerError,
            }
        }
    }
}

#[get("/simulationResult/<id>", format = "application/json")]
pub fn get_simulation(id: String) -> ApiResponse {
    match model::SimulationResult::get(id) {
        Ok(val) => {
            ApiResponse {
                json: json!(val),
                status: Status::Ok,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!("api key not found"),
                status: Status::NotFound,
            }
        }
    }
}

#[delete("/simulationResult/<id>", format = "application/json")]
pub fn delete_simulation(id: String) -> ApiResponse {
    match model::SimulationResult::delete(id) {
        Ok(val) => {
            ApiResponse {
                json: json!(val),
                status: Status::Ok,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!("api key not found"),
                status: Status::NotFound,
            }
        }
    }
}

#[put("/simulationResult/<id>/invalidate", format = "application/json", data = "<result>")]
pub fn invalidate_simulation(id: String, result: Json<model::SimulationResult>) -> ApiResponse {
    match result.invalidate(id) {
        Ok(val) => {
            ApiResponse {
                json: json!(val),
                status: Status::Ok,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!("api key not found"),
                status: Status::NotFound,
            }
        }
    }
}