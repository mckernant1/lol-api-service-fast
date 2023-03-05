use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use color_eyre::{eyre::Context, Result};
use lol_esports_api::models::Match;
use serde_dynamo::from_item;
use tokio_stream::{Stream, StreamExt};

use crate::util::MATCHES_TABLE_NAME;

pub struct MatchService {
    ddb: Client,
}

impl MatchService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }
}

impl MatchService {
    pub async fn get_matches_for_tournament(
        &self,
        tournament_id: String,
    ) -> impl Stream<Item = Result<Match>> {
        self.ddb
            .query()
            .table_name(MATCHES_TABLE_NAME)
            .key_condition_expression("tournamentId = :desiredTourney")
            .expression_attribute_values(":desiredTourney", AttributeValue::S(tournament_id))
            .into_paginator()
            .items()
            .send()
            .map(|it| {
                let it = it?;
                from_item::<HashMap<String, AttributeValue>, Match>(it.clone())
                    .wrap_err(format!("Failed to convert to Tournament {:?}", it))
            })
    }
}
