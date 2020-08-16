use crate::so::Solution;

use actix_web::{web, HttpResponse};

use utils::tools;

pub fn mod_config_test(cfg: &mut web::ServiceConfig) {
    let test_value = tools::test();
    let c = Solution::longest_common_prefix(vec![
        String::from("hello"),
        String::from("heooo"),
        String::from("hec"),
        String::from("he"),
    ]);

    cfg.service(
        web::resource("/test")
            .route(web::get().to(move || {
                HttpResponse::Ok().body(format!(
                    "test demo: test v -> {:?} logest -> {:?}",
                    test_value, c
                ))
            }))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
