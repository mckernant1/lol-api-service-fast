use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use chrono::{Duration, Utc};
use lol_esports_api::models::Tournament;

use crate::{
    error::LolEsportsApiError,
    svc::{league::LeagueService, tournament::TournamentService},
    util::{tournament_ext::TournamentExt, Response},
};

#[get("/ongoing-tournaments")]
pub async fn get_ongoing_tournaments(tournament_service: Data<TournamentService>) -> Response {
    match tournament_service.get_all_tournaments().await {
        Ok(tourneys) => {
            let now = Utc::now().naive_utc().date();
            let tourneys: Vec<Tournament> = tourneys
                .into_iter()
                .filter(|tourney| {
                    tourney.start_date().map(|it| it < now).unwrap_or(false)
                        && tourney.end_date().map(|it| it > now).unwrap_or(false)
                })
                .collect();
            Ok(HttpResponse::Ok().json(tourneys))
        }
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}

#[get("/tournament/{tournamentId}")]
pub async fn get_tournament(
    tournament_id: Path<String>,
    tournament_service: Data<TournamentService>,
) -> Response {
    let tournament_id = tournament_id.to_string();
    match tournament_service
        .get_tournament(tournament_id.clone())
        .await
    {
        Ok(Some(tourny)) => Ok(HttpResponse::Ok().json(tourny)),
        Ok(None) => Err(LolEsportsApiError::not_found(format!(
            "Tournament with id {} not found",
            tournament_id
        ))),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}

#[get("/tournaments/{leagueId}")]
pub async fn get_tournaments_for_league(
    league_id: Path<String>,
    league_service: Data<LeagueService>,
    tournament_service: Data<TournamentService>,
) -> Response {
    let league_id = league_id.to_string();

    match league_service.get_league(league_id.clone()).await {
        Err(e) => return Err(LolEsportsApiError::internal_error(e)),
        Ok(None) => {
            return Err(LolEsportsApiError::not_found(format!(
                "League id {} does not exist",
                league_id.clone()
            )))
        }
        Ok(_) => {}
    };

    match tournament_service
        .get_tournaments_for_league(league_id)
        .await
    {
        Ok(tourneys) => Ok(HttpResponse::Ok().json(tourneys)),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}

#[get("/most-recent-tournament/{leagueId}")]
pub async fn get_most_recent_tournament(
    league_id: Path<String>,
    tournament_service: Data<TournamentService>,
    league_service: Data<LeagueService>,
) -> Response {
    let league_id = league_id.clone();

    match league_service.get_league(league_id.clone()).await {
        Err(e) => return Err(LolEsportsApiError::internal_error(e)),
        Ok(None) => {
            return Err(LolEsportsApiError::not_found(format!(
                "League id {} does not exist",
                league_id.clone()
            )))
        }
        Ok(_) => {}
    };

    let tourneys = match tournament_service.get_all_tournaments().await {
        Ok(tourneys) => tourneys,
        Err(e) => return Err(LolEsportsApiError::internal_error(e)),
    };

    let now = Utc::now().naive_utc().date();

    let mut tourneys: Vec<&Tournament> = tourneys
        .iter()
        .filter(|tourney| match tourney.start_date() {
            Some(start) => (start - Duration::days(7)) < now,
            None => false,
        })
        .collect();

    if league_id.to_ascii_lowercase() == "wcs" {
        tourneys = tourneys
            .into_iter()
            .filter(|it| it.is_official.unwrap_or(false))
            .collect();
    }

    let tourney = tourneys
        .iter()
        .find(|it| it.is_ongoing())
        .or_else(|| tourneys.first());

    Ok(HttpResponse::Ok().json(tourney))
}
