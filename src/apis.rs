use chesscom_openapi::{
    apis::{
        configuration,
        default_api::{get_player_profile, get_player_stats, ApiError},
        urlencode, Error, ResponseContent,
    },
    models::PlayerStats,
};

use chrono::{DateTime, Duration, Utc};
use futures::{
    stream::{self},
    StreamExt, TryFutureExt,
};

// futures::stream::Fold
use std::{option::Option, str::FromStr};

use crate::{date_iter::get_all_month_years_from_now, models::Games};

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

pub async fn get_games(username: &str) -> Result<Vec<Games>, Box<dyn std::error::Error>> {
    let first_game_str = "2022-03-18T23:59:60.234567+05:00";
    let first_game_date = DateTime::<Utc>::from_str(first_game_str)?;
    let dates = get_all_month_years_from_now(first_game_date, None);

    let y = stream::iter(dates)
        .fold(vec![], |mut all_games, (month, year)| async move {
            let conf = chesscom_openapi::apis::configuration::Configuration::default();
            let games_future =
                get_chess_games_for_month_local(&conf, username, &year, &month).await;
            all_games.push(games_future);
            all_games
        })
        .await;

    let games = y
        .into_iter()
        .collect::<Result<Vec<Games>, chesscom_openapi::apis::Error<ApiError>>>()?;

    Ok(games)
}

async fn get_profile(username: &str) -> Result<PlayerStats, Box<dyn std::error::Error>> {
    let conf = chesscom_openapi::apis::configuration::Configuration::default();
    let profile = get_player_profile(&conf, username).await?;
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

    let stats = get_player_stats(&conf, username).await?;
    Ok(stats)
}
