use chesscom_openapi::apis::{
    configuration,
    default_api::ApiError,
    urlencode, Error, ResponseContent,
};
use crate::{date_iter::get_all_month_years_from_now, models::Games};

use chrono::{DateTime, Utc};
use futures::{
  stream::{self},
  StreamExt,
};

use std::{option::Option, str::FromStr};

use super::GamesSource;
pub struct ChessCommGamesSource {
  pub username: String,
  pub from_month_string: String,
}

async fn get_games_for_month(
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
      local_var_req_builder = local_var_req_builder.header("user-agent", local_var_user_agent.clone());
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


async fn get_all_games(username: &str, from_month_string: &str) -> Result<Vec<Games>, Box<dyn std::error::Error>> {
  let from_month_string_date = DateTime::<Utc>::from_str(from_month_string)?;
  let dates = get_all_month_years_from_now(from_month_string_date, None);
  println!("dates {:#?}", dates);

  let y = stream::iter(dates)
      .fold(vec![], |mut all_games, (month, year)| async move {
          let conf = chesscom_openapi::apis::configuration::Configuration::default();
          let games_future =
          get_games_for_month(&conf, username, &year, &month).await;
          all_games.push(games_future);
          all_games
      })
      .await;

  let games = y
      .into_iter()
      .collect::<Result<Vec<Games>, chesscom_openapi::apis::Error<ApiError>>>()?;

  Ok(games)
}

impl GamesSource for ChessCommGamesSource {
  async fn get_games(&self) -> Result<Vec<Games>, Box<dyn std::error::Error>> {
      let username = &self.username;
      get_all_games(username, &self.from_month_string).await
  }

}