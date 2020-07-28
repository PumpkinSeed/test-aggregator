// use rocket::http::Status;
// use rocket::response::status;
// use rocket_contrib::json::Json;
// use std::vec;
//
// mod sim_model {
//     include!("../models/simulation_result.rs");
// }
//
// mod user_model {
//     include!("../models/user.rs");
// }
//
// pub fn routes() -> std::vec::Vec<rocket::Route> {
//     routes!(send_mail)
// }
//
// #[put("/send-mail/<id>", format = "application/json", data = "<result>")]
// pub fn send_mail(id: String, result: Json<sim_model::SimulationResult>) -> status::Accepted<String> {
//     if result.status == "error" || result.status == "failed" { //TODO use enums
//         match user_model::User.get_by_git_user(result.triggered_by) {
//             Ok(user) => {
//                 match user.notification {
//                     Some(notification) => {
//                         if notification {
//                             // send_mail()
//                         }
//                     }
//                     None => return status::Accepted(None)
//                 }
//                 // status::Accepted(Some("success".to_string()))
//             },
//             Err(err) => {
//                 println!("{}", err.as_str());
//                 return status::Accepted(None)
//             }
//         }
//     }
//     status::Accepted(Some("success".to_string()))
// }