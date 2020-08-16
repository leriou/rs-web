pub struct DemoCtl;

use actix_web::{HttpResponse, Responder};

impl DemoCtl {
    pub fn New() -> Self {
        DemoCtl {}
    }

    pub async fn print_test(&self) -> impl Responder {
        HttpResponse::Ok().body(format!("hello ctl print {:?}", "success"))
    }
}
