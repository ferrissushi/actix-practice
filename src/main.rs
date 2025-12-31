use std::io::Result;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let env = env_logger::Env::new().default_filter_or("info");
    env_logger::init_from_env(env);

    log::info!("Starting HTTP Server");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello World" }))
            .service(web::resource("/").to(|| async { "Welcome to my first web server" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {

    use actix_web::{body::to_bytes, dev::Service, test, web, App, Error};
    use awc::http;

    use crate::index;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = app.call(req).await?;

        assert_eq!(res.status(), http::StatusCode::OK);

        // I don't know what the fuck is this
        let response_body = res.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }
}
