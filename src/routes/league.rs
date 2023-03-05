use actix_web::{
    get,
    web::{self, Data, Path},
    HttpResponse, Scope,
};

use crate::error::LolEsportsApiError;
use crate::svc::league::LeagueService;
use crate::util::Response;

pub fn league_endpoints() -> Scope {
    web::scope("/leagues")
        .service(get_all_leagues)
        .service(get_league)
}

#[get("")]
async fn get_all_leagues(league_service: Data<LeagueService>) -> Response {
    match league_service.get_all_leagues().await {
        Ok(leagues) => Ok(HttpResponse::Ok().json(leagues)),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}

#[get("/{leagueId}")]
async fn get_league(league_id: Path<String>, league_service: Data<LeagueService>) -> Response {
    let league_id = league_id.to_string();
    match league_service.get_league(league_id.clone()).await {
        Ok(Some(league)) => Ok(HttpResponse::Ok().json(league)),
        Ok(None) => Err(LolEsportsApiError::not_found(format!(
            "leagueId '{league_id}' does not exist"
        ))),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}
