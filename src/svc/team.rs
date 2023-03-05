use aws_sdk_dynamodb::Client;
use color_eyre::{eyre::Context, Result};
use lol_esports_api::models::Team;
use serde_dynamo::from_item;

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
}
