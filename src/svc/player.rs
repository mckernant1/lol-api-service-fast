use aws_sdk_dynamodb::{model::AttributeValue, Client};
use color_eyre::{eyre::Context, Result};
use lol_esports_api::models::Player;
use serde_dynamo::from_items;
use tokio_stream::StreamExt;

use crate::util::{PLAYERS_TABLE_NAME, PLAYERS_TABLE_TEAM_INDEX};

pub struct PlayerService {
    ddb: Client,
}

impl PlayerService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl PlayerService {
    pub async fn get_players_on_team(&self, team_id: String) -> Result<Vec<Player>> {
        let items = self
            .ddb
            .query()
            .table_name(PLAYERS_TABLE_NAME)
            .index_name(PLAYERS_TABLE_TEAM_INDEX)
            .key_condition_expression("teamId = :desiredTeam")
            .expression_attribute_values(":desiredTeam", AttributeValue::S(team_id))
            .into_paginator()
            .items()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?;

        from_items(items).wrap_err("Failed to convert to players")
    }
}
