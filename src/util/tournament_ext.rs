use chrono::{NaiveDate, Utc};
use lol_esports_api::models::Tournament;

const TOURNEY_DATE_FORMAT: &str = "%Y-%m-%d";

pub trait TournamentExt {
    fn start_date(&self) -> Option<NaiveDate>;

    fn end_date(&self) -> Option<NaiveDate>;

    fn is_ongoing(&self) -> bool;
}

impl TournamentExt for Tournament {
    fn start_date(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(self.start_date.clone()?.as_str(), TOURNEY_DATE_FORMAT).ok()
    }

    fn end_date(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(self.end_date.clone()?.as_str(), TOURNEY_DATE_FORMAT).ok()
    }

    fn is_ongoing(&self) -> bool {
        let now = Utc::now().naive_utc().date();
        self.start_date().unwrap_or(NaiveDate::MIN) <= now
            && now <= self.end_date().unwrap_or(NaiveDate::MIN)
    }
}
