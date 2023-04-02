use chesscom_openapi::{
    apis::{
        configuration,
        default_api::{get_player_profile, get_player_stats, ApiError},
        urlencode, Error, ResponseContent,
    },
    models::PlayerStats,
};


use std::{thread, time::{self}, task, future};
use chrono::{Duration, Utc, DateTime};
use futures::{future::{ok, try_join_all}, stream::{FuturesUnordered, FuturesOrdered, Fold, self}, StreamExt, Future, TryFutureExt};
// futures::stream::Fold
use std::{option::Option, str::FromStr};

use crate::{models::Games, date_iter::get_all_month_years_from_now};
use serde_json::json;

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
    println!("BEFORE {}", local_var_uri_str);

    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    println!("AFTER {}", local_var_uri_str);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;
    // println!("{:#?}", local_var_content);
    // let obj = json!(local_var_content);
    // println!(
    //     "{:#}",
    //     serde_json::to_string_pretty(&local_var_content).unwrap()
    // );
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

    let y = stream::iter(dates).fold(vec![], |mut all_games, (month, year)| async move {
        let conf = chesscom_openapi::apis::configuration::Configuration::default();
        let games_future = get_chess_games_for_month_local(&conf, &username, &year, &month).await;
        all_games.push(games_future);
        all_games
    }).await;

    // let combined_task: Box<_> =
    // dates.into_iter().fold(Box::new(ok(())), |acc, (year, month)| {
    //     let games_future = get_chess_games_for_month_local(&conf, &username, &year, &month);
    //     Box::new(acc.and_then(|_| games_future))
    // });
    // let game_futures = stream::iter(dates).fold(vec![], |mut all_games, (month, year)| {
    //     let games = get_chess_games_for_month_local(&conf, &username, year, month);
    //     let games2 = get_chess_games_for_month_local(&conf, &username, year, month);
    //     // games.
    //     all_games.push(games);
    //     future::ready(all_games)
    // }).await;

    

    // let all_game_futures = dates.iter().fold(vec![], |mut all_games, (month, year)| {
    //     println!("IN FOLD BEFORE {} {}", year, month);
    //     let games = get_chess_games_for_month_local(&conf, &username, year, month);
    //     println!("IN FOLD AFTER {} {}", year, month);
    //     all_games.push(games);
    //     all_games
    // });
    // let x = futures::future::join_all(y).await;
    // // let games_futures = FuturesOrdered::from_iter(all_game_futures);
    // // iter_ok::<_, ()>(all_game_futures).for_each(|f| f);
    // let x = games_futures.collect::<Vec<_>>().await;
    let games = y.into_iter().collect::<Result<Vec<Games>, chesscom_openapi::apis::Error<ApiError>>>()?;

    // let results: Vec<Games> = try_join_all(game_futures).await?;
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
