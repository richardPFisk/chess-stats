use super::models::Games;
pub mod chess_com_source;

pub trait GamesSource {
  async fn get_games(&self) -> Result<Vec<Games>, Box<dyn std::error::Error>>;
}
