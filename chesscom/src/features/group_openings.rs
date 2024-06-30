use std::collections::{BTreeMap, HashMap};
use fp_core::functor::Functor;
use fp_core::chain::Chain;

use crate::models::CompletedGame;
use crate::string_util::{get_child_to_parent_map, get_parent_child_strings};
use super::game_opening::get_all_openings;

pub fn group_by_opening_by_root_opening(username: &str, games: &[CompletedGame]) -> Vec<(String, Vec<CompletedGame>)> {
    let openings = get_all_openings(username, games);
    
    let grouped_games: HashMap<String, Vec<CompletedGame>> = openings.iter()
        .map(|opening| opening.name.clone())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|opening| (opening, Vec::new()))
        .collect();

    let parent_child_openings: BTreeMap<String, Vec<String>> = get_parent_child_strings(openings.iter().map(|o| o.name.clone()).collect());
    let child_to_parent_map = get_child_to_parent_map(openings.iter().map(|o| o.name.clone()).collect());

    let grouped_games = games.iter()
        .fold(grouped_games, |mut acc, game| {
            get_all_openings(username, &[game.clone()])
                .first()
                .chain(|opening| {
                    child_to_parent_map.get(&opening.name)
                        .map(|parent| (parent.clone(), game.clone()))
                })
                .map(|(parent, game)| {
                    acc.entry(parent).or_insert_with(Vec::new).push(game);
                });
            acc
        });

    let mut gg: Vec<(String, Vec<CompletedGame>)> = grouped_games.into_iter().collect();
    gg.sort_by(|g1, g2| g1.0.len().cmp(&g2.0.len()));
    gg
}