use std::fs::File;

use crate::{
    features::{game_opening::game_to_opening, result::result},
    models::CompletedGame,
};





use serde::{Deserialize, Serialize};
// use ndarray::{array, Array1, Array2};
// use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug)]
struct LinfaFeatures {
    opening: String,
    result: String,
}

// impl<'a> Records<'a, Array2<f64>> for LinfaFeatures {
//   type Record = (Array2<f64>, Array1<f64>);

//   fn records(&'a self) -> Self::Record {
//       let mut feature_data = Vec::new();
//       let mut target_data = Vec::new();

//       // Extract feature data from the moves
//       for (idx, mv) in self.moves.iter().enumerate() {
//           let feature_vec = match mv.as_str() {
//               "e4" => array![1.0, 0.0, 0.0, 0.0],
//               "d4" => array![0.0, 1.0, 0.0, 0.0],
//               "Nf3" => array![0.0, 0.0, 1.0, 0.0],
//               "c4" => array![0.0, 0.0, 0.0, 1.0],
//               _ => continue,
//           };
//           feature_data.push(feature_vec);
//           target_data.push(idx as f64);
//       }

//       (Array2::from_shape_vec((feature_data.len(), 4), feature_data).unwrap(), Array1::from_shape_vec(target_data.len(), target_data).unwrap())
//   }
// }

pub fn get_linfa_tree(
    username: &str,
    games: &Vec<CompletedGame>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _features = games
        .iter()
        .map(|g| {
            let o = game_to_opening(username, g);
            let r = result(username, g.clone());

            LinfaFeatures {
                opening: o.map(|op|op.name).unwrap_or("".to_string()),
                result: r.to_string(),
            }
        })
        .collect::<Vec<_>>();

    // let data = features
    //   .iter()
    //   .map(|d| Ok(array![d.opening.clone(), d.result.clone()]))
    //   .collect::<Result<Vec<Array1<String>>, Box<dyn std::error::Error>>>();

    let data = ndarray::arr2(&[
        [35.0, 1.0, 50000.0, 10.0, 120.0, 3.0],
        [45.0, 0.0, 75000.0, 20.0, 240.0, 2.0],
        [25.0, 1.0, 30000.0, 5.0, 60.0, 1.0],
        [30.0, 0.0, 40000.0, 15.0, 180.0, 4.0],
    ]);

    // let d = Dataset::new(data, None);

    // let dataset = DatasetBase {
    //     records: data,
    //     targets: vec!["asdf1", "asdf2", "asdf3", "asdf4", "5", "6"].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    //     weights: vec![1.0, 1.0, 1.0, 1.0].into(),
    //     feature_names: vec!["asdf1", "asdf2", "asdf3", "asdf4", "5", "6"].iter().map(|s| s.to_string()).collect(),
    // };
    // dataset.with_labels();

    // let tree = DecisionTree::params()
    //     .split_quality(linfa_trees::SplitQuality::Gini)
    //     .min_weight_split(1.0)
    //     .min_weight_leaf(1.0)
    //     .max_depth(Some(2))
    //     .fit(&dataset)?;

    let _tikz =
        File::create("/Users/richardfisk/projects/chess-stats/decision_tree_example.tex").unwrap();
    // tikz.write_all(tree.export_to_tikz().with_legend().to_string().as_bytes())
    //     .unwrap();
    Ok(())
}

// Create a column for each possible value of the feature, and set the value to 1 if the feature has that value, and 0 otherwise.
