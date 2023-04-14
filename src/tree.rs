use crate::{
    features::{opening, result},
    models::CompletedGame,
};
// use linfa::{traits::Fit, DatasetBase};
// use linfa_trees::DecisionTree;
use ndarray::{array, Array1, Array2};
use serde::{Deserialize, Serialize};

use linfa::dataset::{DatasetBase, Records};
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

pub fn get_linfa_tree(username: &str, games: Vec<CompletedGame>) {
    let features = games
        .iter()
        .map(|g| {
            let o = opening(&g);
            let r = result(username, g.clone());

            LinfaFeatures {
                opening: o.unwrap_or("".to_string()),
                result: r.to_string(),
            }
        })
        .collect::<Vec<_>>();

    //   let data = features
    //     .iter()
    //     .map(|d| Ok(array![d.opening.clone(), d.result.clone()]))
    //     .collect::<Result<Vec<Array1<String>>, Box<dyn std::error::Error>>>();
    //   let dataset = DatasetBase::from(data);
    // let tree = DecisionTree::params()
    //     .split_quality(linfa_trees::SplitQuality::Gini)
    //     .max_depth(Some(2))
    //     .fit(&data);
}

// Create a column for each possible value of the feature, and set the value to 1 if the feature has that value, and 0 otherwise.
