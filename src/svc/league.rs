use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use color_eyre::{eyre::Context, eyre::Result};
use lol_esports_api::models::League;
use serde_dynamo::from_item;
use tokio_stream::{Stream, StreamExt};

use crate::{error::LolEsportsApiError, util::LEAGUES_TABLE_NAME};

pub struct LeagueService {
    ddb: Client,
}

impl LeagueService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl LeagueService {
    pub async fn league_exists(&self, league_id: String) -> Result<()> {
        match self.get_league(league_id.clone()).await? {
            None => {
                return Err(LolEsportsApiError::not_found(format!(
                    "League id {} does not exist",
                    league_id.clone()
                ))
                .into())
            }
            Some(_) => Ok(()),
        }
    }

    pub async fn get_all_leagues(&self) -> impl Stream<Item = Result<League>> {
        self.ddb
            .scan()
            .table_name(LEAGUES_TABLE_NAME)
            .into_paginator()
            .items()
            .send()
            .map(|it| {
                let it = it?;
                from_item::<HashMap<String, AttributeValue>, League>(it.clone())
                    .wrap_err(format!("Failed to convert to Tournament {:?}", it))
            })
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
