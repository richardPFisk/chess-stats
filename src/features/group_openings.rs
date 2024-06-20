
use std::collections::{BTreeMap, HashMap};

use crate::{models::CompletedGame, string_util::{get_child_to_parent_map, get_parent_child_strings}};

use super::{game_opening::{get_all_openings, opening}, username_colour::username_colour};

pub fn group_by_opening_by_root_opening(username: &str, games: &[CompletedGame]) -> Vec<(String, Vec<CompletedGame>)> {
    
    let game_to_opening_name: Box<dyn Fn(&CompletedGame) -> Option<String>> = Box::new(|game: &CompletedGame| {
        let my_colour = username_colour(username, game);
        let original_opening = opening(game).unwrap_or_else(|| "Unknown".to_string());
        
        Some(format!("{} ({})", original_opening, my_colour.as_str()))
    });
  
    let mut grouped_games: HashMap<String, Vec<CompletedGame>> = HashMap::new();
    let openings = get_all_openings(username, games);
    let parent_child_openings: BTreeMap<String, Vec<String>> =
        get_parent_child_strings(openings.clone());
  
    for (opening_parent, _openings) in parent_child_openings {
        grouped_games.insert(opening_parent, vec![]);
    }
    let _child_to_parent_map = get_child_to_parent_map(openings);
  
    for game in games {
        let opening_name = game_to_opening_name(&game).unwrap();
        if let Some(games_in_group) = grouped_games.get_mut(&opening_name) {
            games_in_group.push(game.clone());
        }
    }
  
    let mut gg: Vec<(String, Vec<CompletedGame>)> = grouped_games.into_iter().collect();
    gg.sort_by(|g1, g2| g1.0.len().cmp(&g2.0.len()));
    gg
}