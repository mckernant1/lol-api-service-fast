pub mod tournament_ext;

use actix_web::HttpResponse;

use crate::error::LolEsportsApiError;

pub type Response = Result<HttpResponse, LolEsportsApiError>;

pub const MATCHES_TABLE_NAME: &str = "Matches";
pub const LEAGUES_TABLE_NAME: &str = "Leagues";
pub const PLAYERS_TABLE_NAME: &str = "Players";
pub const PLAYERS_TABLE_TEAM_INDEX: &str = "teamId-id-index";
pub const TEAMS_TABLE_NAME: &str = "Teams";
pub const TOURNAMENTS_TABLE_NAME: &str = "Tournaments";
pub const TOURNAMENTS_INDEX: &str = "tournamentId-index";
