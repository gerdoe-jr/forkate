use std::env;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_lab::web::redirect;

mod certificate;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let prod_env = env::var("PRODUCTION_ENV").is_ok();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(redirect("/", "/home"))
            .service(web::resource("/home").route(web::get().to(templates::home)))
            .service(web::resource("/blog").route(web::get().to(templates::blog)))
            .service(web::resource("/articles").route(web::get().to(templates::articles)))
            .service(web::resource("/contacts").route(web::get().to(templates::contacts)))
            .service(
                fs::Files::new("static/", "static/")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(
                fs::Files::new(
                    "/.well-known/acme-challenge/",
                    "./.well-known/acme-challenge/",
                )
                .show_files_listing()
                .use_last_modified(true),
            )
            .default_service(web::route().to(templates::not_found))
    });

    if !prod_env {
        // start local http server
        server.bind(("127.0.0.1", 8000))?.run().await
    } else {
        let contact_email = env::var("CONTACT_EMAIL").unwrap();
        let primary_name = env::var("PRIMARY_NAME").unwrap();
        // start public https server
        let config = certificate::load_rustls_config(&contact_email, &primary_name, prod_env);

        server.bind_rustls(("0.0.0.0", 443), config)?.run().await
    }
}
