use actix_web::{
    get,
    web::{self, Data, Path},
    HttpResponse, Scope,
};
use color_eyre::Result;
use tokio_stream::StreamExt;

use crate::{svc::player::PlayerService, util::Response};

pub fn player_endpoints() -> Scope {
    web::scope("/players").service(get_players_on_team)
}

#[get("/{teamId}")]
async fn get_players_on_team(
    team_id: Path<String>,
    players_service: Data<PlayerService>,
) -> Response {
    let team_id = team_id.to_string();

    let players = players_service
        .get_players_on_team(team_id)
        .await
        .collect::<Result<Vec<_>>>()
        .await?;
    Ok(HttpResponse::Ok().json(players))
}
