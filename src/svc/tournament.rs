use aws_sdk_dynamodb::{model::AttributeValue, Client};
use color_eyre::{eyre::Context, Result};
use lol_esports_api::models::Tournament;
use serde_dynamo::{from_item, from_items};
use tokio_stream::StreamExt;

use crate::util::{TOURNAMENTS_INDEX, TOURNAMENTS_TABLE_NAME};

pub struct TournamentService {
    ddb: Client,
}

impl TournamentService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl TournamentService {
    pub async fn get_tournaments_for_league(&self, league_id: String) -> Result<Vec<Tournament>> {
        let items = self
            .ddb
            .query()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .key_condition_expression("leagueId = :desiredLeague")
            .expression_attribute_values(":desiredLeague", AttributeValue::S(league_id))
            .into_paginator()
            .items()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?;

        from_items(items).wrap_err("Failed to convert Tournament")
    }

    pub async fn get_tournament(&self, tournament_id: String) -> Result<Option<Tournament>> {
        let item = self
            .ddb
            .query()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .index_name(TOURNAMENTS_INDEX)
            .key_condition_expression("tournamentId = :desiredTournament")
            .expression_attribute_values(":desiredTournament", AttributeValue::S(tournament_id))
            .into_paginator()
            .items()
            .send()
            .next()
            .await;

        match item {
            Some(Ok(item)) => {
                from_item(item.clone()).wrap_err(format!("Failed to convert {:?} to League", item))
            }
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub async fn get_all_tournaments(&self) -> Result<Vec<Tournament>> {
        let items = self
            .ddb
            .scan()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .into_paginator()
            .items()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?;

        from_items(items.clone()).wrap_err(format!("Failed to convert to Tournament {:?}", items))
    }
}
