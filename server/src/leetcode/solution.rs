extern crate actix_web;

use actix_web::{web, HttpResponse};

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        println!("i get it {:?}", strs);
        strs[0].clone()
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("right heii")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
