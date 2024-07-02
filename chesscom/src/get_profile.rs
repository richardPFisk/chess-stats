use chesscom_openapi::{
    apis::{
        default_api::{get_player_profile, get_player_stats},
    },
    models::PlayerStats,
};

use chrono::{Duration, Utc};


// use crate::{date_iter::get_all_month_years_from_now, models::Games};

pub async fn get_profile(username: &str) -> Result<PlayerStats, Box<dyn std::error::Error>> {
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
