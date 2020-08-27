use std::vec;

use rocket::http::{ContentType, Header, Status};
use rocket_contrib::json::Json;

use crate::models::{api_key as model,response::ApiResponse};

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes!(create,update,get_all,get_one,delete)
}

#[post("/apiKey", format = "application/json", data = "<api_key>")]
pub fn create(api_key: Json<model::ApiKey>) -> ApiResponse {
    match api_key.insert() {
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

#[put("/apiKey/<id>", format = "application/json", data = "<api_key>")]
pub fn update(id: String, api_key: Json<model::ApiKey>) -> ApiResponse {
    match api_key.update(id) {
        Ok(()) => {
            ApiResponse {
                json: json!(""),
                status: Status::Created,
            }
        }
        Err(err) => {
            ApiResponse {
                json: json!(format!("{}",err)),
                status: Status::NotFound,
            }
        }
    }
}

#[get("/apiKey", format = "application/json")]
pub fn get_all() -> ApiResponse {
    match model::ApiKey::get_all() {
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

#[get("/apiKey/<id>", format = "application/json")]
pub fn get_one(id: String) -> ApiResponse {
    match model::ApiKey::get(id) {
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

#[delete("/apiKey/<id>", format = "application/json")]
pub fn delete(id: String) -> ApiResponse {
    match model::ApiKey::delete(id) {
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