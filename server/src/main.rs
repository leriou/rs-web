mod controller;
mod leetcode;
mod middware;
mod service;

use actix_web::{get, web, App, HttpServer, Responder};
use controller::demo_ctl;
use leetcode::solution as so;
use listenfd::ListenFd;

#[get("/{id}/{name}/idx")]
async fn path_param_test(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .service(path_param_test)
            .service(web::scope("/api").configure(demo_ctl::mod_config_test))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
