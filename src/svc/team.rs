use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use color_eyre::{eyre::Context, Result};
use lol_esports_api::models::Team;
use serde_dynamo::{from_item};
use tokio_stream::{Stream, StreamExt};

use crate::util::TEAMS_TABLE_NAME;

pub struct TeamService {
    ddb: Client,
}

impl TeamService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl TeamService {
    pub async fn get_team(&self, team_id: String) -> Result<Option<Team>> {
        let resp = self
            .ddb
            .get_item()
            .table_name(TEAMS_TABLE_NAME)
            .key(
                "teamId",
                aws_sdk_dynamodb::model::AttributeValue::S(team_id),
            )
            .send()
            .await?;

        if let Some(item) = resp.item() {
            from_item(item.clone()).wrap_err(format!("Failed to convert {:?} to League", item))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_teams(&self) -> impl Stream<Item = Result<Team>> {
        self.ddb
            .scan()
            .table_name(TEAMS_TABLE_NAME)
            .into_paginator()
            .items()
            .send()
            .map(|it| {
                let it = it?;
                from_item::<HashMap<String, AttributeValue>, Team>(it.clone())
                    .wrap_err(format!("Failed to convert to Team {:?}", it))
            })
    }
}
