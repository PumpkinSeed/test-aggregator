use serde_json::Value;
use serde::{Deserialize, Serialize};

use rocket::Outcome;
use rocket::http::{ContentType, Status};
use rocket::request::{self, Request, FromRequest};
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}