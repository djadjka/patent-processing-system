mod patents;
mod requests;

use actix_web::web;

pub fn concated_resources (cfg: &mut web::ServiceConfig) {
    patents::append_resources(cfg);
}
