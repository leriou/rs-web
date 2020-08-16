mod leetcode;

extern crate redis;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use leetcode::solution::config;
use leetcode::solution::Solution;
use listenfd::ListenFd;

use redis::Commands;

async fn logest_prefix() -> impl Responder {
    let c = Solution::longest_common_prefix(vec![
        String::from("hello"),
        String::from("heooo"),
        String::from("hec"),
        String::from("he"),
    ]);
    HttpResponse::Ok().body(format!("common {:?}", c))
}

#[get("/{id}/{name}/idx")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

async fn index3() -> impl Responder {
    let c = fetch_an_integer().unwrap_or(100);
    HttpResponse::Ok().body(format!("Hey -> {}", c))
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(logest_prefix))
            .service(index)
            .service(web::scope("/api").configure(config))
            .service(web::scope("/test").route("/redis", web::get().to(index3)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
