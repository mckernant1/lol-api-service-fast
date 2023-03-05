use actix_web::{
    get,
    web::{self, Data, Path},
    HttpResponse, Scope,
};

use crate::{error::LolEsportsApiError, svc::team::TeamService, util::Response};

pub fn team_endpoints() -> Scope {
    web::scope("/teams")
        .service(get_all_teams)
        .service(get_team)
}

#[get("")]
async fn get_all_teams(team_service: Data<TeamService>) -> Response {
    match team_service.get_all_teams().await {
        Ok(teams) => Ok(HttpResponse::Ok().json(teams)),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}

#[get("/{teamId}")]
async fn get_team(team_id: Path<String>, team_service: Data<TeamService>) -> Response {
    let team_id = team_id.to_string();
    match team_service.get_team(team_id.clone()).await {
        Ok(Some(team)) => Ok(HttpResponse::Ok().json(team)),
        Ok(None) => Err(LolEsportsApiError::not_found(format!(
            "Could not find team with id {}",
            team_id
        ))),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}
