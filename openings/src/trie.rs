use trie_rs::TrieBuilder;

use crate::models::Opening;

fn a (opening: Opening) {
  let mut trie_builder = TrieBuilder::new();
  
  trie_builder.push(&opening);
  
  let moves = vec!["d4", "Nf6", "c4"];
  let results: Vec<&Opening> = trie_builder.predictive_search(&moves);
  
  for opening in results {
      println!("Matched opening: {:?}", opening.name);
  }
}