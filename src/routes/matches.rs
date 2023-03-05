use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::{
    error::LolEsportsApiError,
    svc::{matches::MatchService, tournament::TournamentService},
    util::Response,
};

#[get("/matches/{tournament_id}")]
async fn get_matches_for_tournament(
    tournament_id: Path<String>,
    match_service: Data<MatchService>,
    tournament_service: Data<TournamentService>,
) -> Response {
    let tournament_id = tournament_id.to_string();

    match tournament_service
        .get_tournament(tournament_id.clone())
        .await
    {
        Ok(None) => {
            return Err(LolEsportsApiError::not_found(format!(
                "Tournament with id {} does not exist",
                tournament_id.clone()
            )))
        }
        Err(e) => return Err(LolEsportsApiError::internal_error(e)),
        Ok(Some(_)) => {}
    };

    match match_service
        .get_matches_for_tournament(tournament_id)
        .await
    {
        Ok(matches) => Ok(HttpResponse::Ok().json(matches)),
        Err(e) => Err(LolEsportsApiError::internal_error(e)),
    }
}
