use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use color_eyre::{eyre::Context, Result};
use lol_esports_api::models::Tournament;
use serde_dynamo::{from_item};
use tokio_stream::{Stream, StreamExt};

use crate::{
    error::LolEsportsApiError,
    util::{TOURNAMENTS_INDEX, TOURNAMENTS_TABLE_NAME},
};

pub struct TournamentService {
    ddb: Client,
}

impl TournamentService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl TournamentService {
    pub async fn tournament_exists(&self, tournament_id: String) -> Result<()> {
        match self.get_tournament(tournament_id.clone()).await {
            Ok(None) => {
                return Err(LolEsportsApiError::not_found(format!(
                    "Tournament with id {} does not exist",
                    tournament_id.clone()
                ))
                .into())
            }
            Err(e) => return Err(LolEsportsApiError::internal_error(e).into()),
            Ok(Some(_)) => Ok(()),
        }
    }

    pub async fn get_tournaments_for_league(
        &self,
        league_id: String,
    ) -> impl Stream<Item = Result<Tournament>> {
        self.ddb
            .query()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .key_condition_expression("leagueId = :desiredLeague")
            .expression_attribute_values(":desiredLeague", AttributeValue::S(league_id))
            .into_paginator()
            .items()
            .send()
            .map(|it| {
                let it = it?;
                from_item::<HashMap<String, AttributeValue>, Tournament>(it.clone())
                    .wrap_err(format!("Failed to convert to Tournament {:?}", it))
            })
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

    pub async fn get_all_tournaments(&self) -> impl Stream<Item = Result<Tournament>> {
        self.ddb
            .scan()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .into_paginator()
            .items()
            .send()
            .map(|it| {
                let it = it?;
                from_item::<HashMap<String, AttributeValue>, Tournament>(it.clone())
                    .wrap_err(format!("Failed to convert to Tournament {:?}", it))
            })
    }
}
