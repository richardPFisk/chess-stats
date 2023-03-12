// #[macro_use]
// extern crate serde_derive;

// extern crate serde;
// extern crate serde_json;
// extern crate url;
// extern crate reqwest;

use chesscom_openapi::{
    apis::{
        configuration,
        default_api::{get_player_profile, get_player_stats, ApiError},
        urlencode, Error, ResponseContent,
    },
    models::{
        completed_game::{Rules, TimeClass},
        PlayerResult, PlayerStats,
    },
};
use std::option::Option as StdOption;
use std::option::Option;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    games: Vec<CompletedGame>,
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
    white: f32,
    black: f32
}

pub async fn get_chess_games_for_month_local(
    configuration: &configuration::Configuration,
    username: &str,
    year: &str,
    month: &str,
) -> Result<Games, Error<ApiError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/pub/player/{username}/games/{year}/{month}",
        configuration.base_path,
        username = urlencode(username),
        year = urlencode(year),
        month = urlencode(month)
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;
    // println!("{:#?}", local_var_content);
    let obj = json!(local_var_content);
    // println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

async fn get_games(username: &str) -> Result<Games, Box<dyn std::error::Error>> {
    let conf = chesscom_openapi::apis::configuration::Configuration::default();
    let games = get_chess_games_for_month_local(&conf, &username, "2023", "02").await?;

    Ok(games)
}

async fn get_profile(username: &str) -> Result<PlayerStats, Box<dyn std::error::Error>> {
    let conf = chesscom_openapi::apis::configuration::Configuration::default();
    let profile = get_player_profile(&conf, &username).await?;
    println!("Username: {}", profile.username);
    println!("Status: {}", profile.status);
    println!("Name: {}", profile.name.as_deref().unwrap_or("<Unknown>"));
    println!("Joined: {}", profile.joined.format("%Y-%m-%d"));

    let time_since_online = Utc::now() - profile.last_online;

    if time_since_online < Duration::hours(1) {
        println!("Last Online: {} mins ago", time_since_online.num_minutes());
    } else if time_since_online < Duration::days(1) {
        println!("Last Online: {} hours ago", time_since_online.num_hours());
    } else {
        println!("Last Online: {} days ago", time_since_online.num_days());
    }

    let stats = get_player_stats(&conf, &username).await?;
    Ok(stats)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let p = get_profile("richardfisk").await?;
    let g = get_games("richardfisk").await?;
    println!("{:#?}", g);
    Ok(())
}
