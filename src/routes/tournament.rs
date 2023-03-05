use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use chrono::{Duration, Utc};
use color_eyre::Result;
use lol_esports_api::models::Tournament;
use tokio_stream::StreamExt;

use crate::{
    error::LolEsportsApiError,
    svc::{league::LeagueService, tournament::TournamentService},
    util::{tournament_ext::TournamentExt, Response},
};

#[get("/ongoing-tournaments")]
pub async fn get_ongoing_tournaments(tournament_service: Data<TournamentService>) -> Response {
    let now = Utc::now().naive_utc().date();
    let tourneys: Vec<Tournament> = tournament_service
        .get_all_tournaments()
        .await
        .filter(|tourney| {
            if let Ok(tourney) = tourney {
                tourney.start_date().map(|it| it < now).unwrap_or(false)
                    && tourney.end_date().map(|it| it > now).unwrap_or(false)
            } else {
                true
            }
        })
        .collect::<Result<Vec<_>>>()
        .await?;
    Ok(HttpResponse::Ok().json(tourneys))
}

#[get("/tournament/{tournamentId}")]
pub async fn get_tournament(
    tournament_id: Path<String>,
    tournament_service: Data<TournamentService>,
) -> Response {
    let tournament_id = tournament_id.to_string();
    match tournament_service
        .get_tournament(tournament_id.clone())
        .await?
    {
        Some(tourny) => Ok(HttpResponse::Ok().json(tourny)),
        None => Err(LolEsportsApiError::not_found(format!(
            "Tournament with id {} not found",
            tournament_id
        ))),
    }
}

#[get("/tournaments/{leagueId}")]
pub async fn get_tournaments_for_league(
    league_id: Path<String>,
    league_service: Data<LeagueService>,
    tournament_service: Data<TournamentService>,
) -> Response {
    let league_id = league_id.to_string();

    league_service.league_exists(league_id.clone()).await?;

    let tourneys = tournament_service
        .get_tournaments_for_league(league_id)
        .await
        .collect::<Result<Vec<_>>>()
        .await?;
    Ok(HttpResponse::Ok().json(tourneys))
}

#[get("/most-recent-tournament/{leagueId}")]
pub async fn get_most_recent_tournament(
    league_id: Path<String>,
    tournament_service: Data<TournamentService>,
    league_service: Data<LeagueService>,
) -> Response {
    let league_id = league_id.to_string();

    league_service.league_exists(league_id.clone()).await?;

    let now = Utc::now().naive_utc().date();

    let tourneys: Vec<Tournament> = tournament_service
        .get_all_tournaments()
        .await
        .filter(|tourney| match tourney {
            Ok(t) if league_id.to_ascii_lowercase() == "wcs" => t.is_official.unwrap_or(false),
            _ => true,
        })
        .filter(|tourney| match tourney.as_ref().map(|it| it.start_date()) {
            Ok(Some(start)) => (start - Duration::days(7)) < now,
            Ok(None) => false,
            Err(_) => true,
        })
        .collect::<Result<Vec<_>>>()
        .await?;

    let first = tourneys.first();
    let tourney = tourneys
        .iter()
        .find(|it| it.is_ongoing())
        .or(first)
        .cloned();
    Ok(HttpResponse::Ok().json(tourney))
}
