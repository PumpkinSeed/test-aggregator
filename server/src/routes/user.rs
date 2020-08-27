use std::vec;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::models::{response::ApiResponse, user as model};

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes!(create,update,get_all,get_one,delete)
}

#[post("/user", format = "application/json", data = "<user>")]
pub fn create(user: Json<model::User>) -> ApiResponse {
    match user.insert() {
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

#[put("/user/<id>", format = "application/json", data = "<user>")]
pub fn update(id:  String, user: Json<model::User>) -> ApiResponse {
    match user.update(id.to_string()) {
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

#[get("/user", format = "application/json")]
pub fn get_all() -> ApiResponse {
    match model::User::get_all() {
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

#[get("/user/<id>", format = "application/json")]
pub fn get_one(id: String) -> ApiResponse {
    match model::User::get(id) {
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

#[delete("/user/<id>", format = "application/json")]
pub fn delete(id: String) -> ApiResponse {
    match model::User::delete(id) {
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