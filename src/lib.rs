pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::web;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.name)
}
