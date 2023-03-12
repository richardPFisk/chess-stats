
use std::option::Option;

use chesscom_openapi::models::completed_game::Rules;

use chesscom_openapi::models::completed_game::TimeClass;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use chrono;
use std::option::Option as StdOption;
use chesscom_openapi::models::PlayerResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub(crate) games: Vec<CompletedGame>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedGame {
    pub white: PlayerResult,
    pub black: PlayerResult,
    pub url: String,
    pub fen: String,
    pub pgn: String,
    // , with = "chrono::serde::ts_seconds_option"
    #[serde(skip_serializing_if = "StdOption::is_none")]
    pub start_time: StdOption<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of the game end
    #[serde(rename = "end_time", with = "chrono::serde::ts_seconds")]
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub rated: bool,
    /// PGN-compliant time control
    #[serde(rename = "time_control")]
    pub time_control: String,
    /// Time-per-move grouping, used for ratings
    #[serde(rename = "time_class")]
    pub time_class: TimeClass,
    /// Game variant information (e.g., \"chess960\")
    pub rules: Rules,
    /// URL pointing to ECO opening (if available)
    #[serde(rename = "eco", skip_serializing_if = "Option::is_none")]
    pub eco: Option<String>,
    /// URL pointing to tournament (if available)
    #[serde(rename = "tournament", skip_serializing_if = "Option::is_none")]
    pub tournament: Option<String>,
    /// URL pointing to team match (if available)
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<String>,
    pub accuracies: Option<Accuracies>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accuracies {
    pub(crate) white: f32,
    pub(crate) black: f32
}
