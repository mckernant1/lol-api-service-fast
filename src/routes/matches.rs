use actix_web::{
    get,
    web::{self, Data, Path},
    HttpResponse, Scope,
};
use color_eyre::Result;
use tokio_stream::StreamExt;

use crate::{
    svc::{matches::MatchService, tournament::TournamentService},
    util::Response,
};

pub fn matches_endpoints() -> Scope {
    web::scope("/matches").service(get_matches_for_tournament)
}

#[get("/{tournament_id}")]
async fn get_matches_for_tournament(
    tournament_id: Path<String>,
    match_service: Data<MatchService>,
    tournament_service: Data<TournamentService>,
) -> Response {
    let tournament_id = tournament_id.to_string();

    tournament_service
        .tournament_exists(tournament_id.clone())
        .await?;

    let matches = match_service
        .get_matches_for_tournament(tournament_id)
        .await
        .collect::<Result<Vec<_>>>()
        .await?;

    Ok(HttpResponse::Ok().json(matches))
}
