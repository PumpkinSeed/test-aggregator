use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::vec;

mod model {
    include!("../models/simulation_result.rs");
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![init_simulation,get_simulations,get_simulation,update_simulation,delete_simulation,invalidate_simulation]
}

#[post("/simulationResult", format = "application/json", data = "<result>")]
pub fn init_simulation(result: Json<model::SimulationResult>) -> status::Accepted<String> {
    match result.insert() {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}

#[put("/simulationResult/<id>", format = "application/json", data = "<result>")]
pub fn update_simulation(id: String, result: Json<model::SimulationResult>) -> status::Accepted<String> {
    match result.update(id) {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}

#[get("/simulationResult", format = "application/json")]
pub fn get_simulations() -> Result<Json<Vec<model::SimulationResult>>, status::NotFound<String>> {
    match model::SimulationResult::get_all() {
        Ok(smt) => {
            return Ok(Json(smt));
        }
        Err(err) => {
            Err(status::NotFound("Simulation results not found".to_string()))
        }
    }
}

#[get("/simulationResult/<id>", format = "application/json")]
pub fn get_simulation(id: String) -> Result<Json<model::SimulationResult>, status::NotFound<String>> {
    match model::SimulationResult::get(id) {
        Ok(smt) => {
            return Ok(Json(smt));
        }
        Err(e) => {
            Err(status::NotFound("Simulation result not found".to_string()))
        }
    }
}

#[delete("/simulationResult/<id>", format = "application/json")]
pub fn delete_simulation(id: String) -> status::Accepted<String> {
    match model::SimulationResult::delete(id) {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}

#[put("/simulationResult/<id>/invalidate", format = "application/json", data = "<result>")]
pub fn invalidate_simulation(id: String, result: Json<model::SimulationResult>) -> status::Accepted<String> {
    match result.invalidate(id) {
        Ok(_) => status::Accepted(Some("success".to_string())),
        Err(err) => {
            println!("{}", err.as_str());
            status::Accepted(None)
        }
    }
}