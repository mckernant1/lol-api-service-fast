use actix_web::{Error, HttpResponse};

use crate::error::LolEsportsApiError;

pub type Response = Result<HttpResponse, LolEsportsApiError>;

pub const MATCHES_TABLE_NAME: &'static str = "Matches";
pub const LEAGUES_TABLE_NAME: &'static str = "Leagues";
pub const PLAYERS_TABLE_NAME: &'static str = "Players";
pub const PLAYERS_TABLE_TEAM_INDEX: &'static str = "teamId-id-index";
pub const TEAMS_TABLE_NAME: &'static str = "Teams";
pub const TOURNAMENTS_TABLE_NAME: &'static str = "Tournaments";
pub const TOURNAMENTS_INDEX: &'static str = "tournamentId-index";
