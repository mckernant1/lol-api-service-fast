use aws_sdk_dynamodb::Client;
use color_eyre::{eyre::Context, eyre::Result};
use lol_esports_api::models::League;
use serde_dynamo::{from_item, from_items};
use tokio_stream::StreamExt;

use crate::util::LEAGUES_TABLE_NAME;

pub struct LeagueService {
    ddb: Client,
}

impl LeagueService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl LeagueService {
    pub async fn get_all_leagues(&self) -> Result<Vec<League>> {
        let items = self
            .ddb
            .scan()
            .table_name(LEAGUES_TABLE_NAME)
            .into_paginator()
            .items()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?;

        from_items(items).wrap_err("Failed to convert to League")
    }

    pub async fn get_league(&self, id: String) -> Result<Option<League>> {
        let resp = self
            .ddb
            .get_item()
            .table_name(LEAGUES_TABLE_NAME)
            .key("leagueId", aws_sdk_dynamodb::model::AttributeValue::S(id))
            .send()
            .await?;

        if let Some(item) = resp.item() {
            from_item(item.clone()).wrap_err(format!("Failed to convert {:?} to League", item))
        } else {
            Ok(None)
        }
    }

}
