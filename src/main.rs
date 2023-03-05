mod dao;
mod error;
mod routes;
mod svc;
mod util;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};

use color_eyre::{eyre::Context, Result};
use dao::get_ddb_client;
use env_logger::Env;
use routes::{league::league_service, not_found};
use svc::league::LeagueService;

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let ddb = get_ddb_client().await;
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(LeagueService::new(ddb.clone())))
            .service(league_service())
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .wrap_err("Server Failed")
}
