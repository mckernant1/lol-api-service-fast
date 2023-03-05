use actix_web::HttpRequest;

use crate::{error::LolEsportsApiError, util::Response};

pub mod league;
pub mod matches;
pub mod player;
pub mod team;
pub mod tournament;

pub async fn not_found(req: HttpRequest) -> Response {
    let path = req.path();
    Err(LolEsportsApiError::page_not_found(path))
}
