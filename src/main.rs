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
use routes::{
    league::league_endpoints,
    matches::matches_endpoints,
    not_found,
    player::player_endpoints,
    team::team_endpoints,
    tournament::{
        get_most_recent_tournament, get_ongoing_tournaments, get_tournament,
        get_tournaments_for_league,
    },
};
use svc::{
    league::LeagueService, matches::MatchService, player::PlayerService, team::TeamService,
    tournament::TournamentService,
};

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(Env::default().default_filter_or("warn"));
    let ddb = get_ddb_client().await;
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(LeagueService::new(ddb.clone())))
            .app_data(Data::new(MatchService::new(ddb.clone())))
            .app_data(Data::new(PlayerService::new(ddb.clone())))
            .app_data(Data::new(TeamService::new(ddb.clone())))
            .app_data(Data::new(TournamentService::new(ddb.clone())))
            .service(league_endpoints())
            .service(matches_endpoints())
            .service(player_endpoints())
            .service(team_endpoints())
            .service(get_tournament)
            .service(get_ongoing_tournaments)
            .service(get_tournaments_for_league)
            .service(get_most_recent_tournament)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .wrap_err("Server Failed")
}
