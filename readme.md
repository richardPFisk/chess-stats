## Todo

### Game concepts
* player focus (api only uses username to relate facets of games to )
  * result
  * colour
  * etc

### Goal
The goal is to create actionable insights.
By highlighting problems and trends, what are the concrete changes that will lead to better outcomes.
Examples:
* Is your French defence getting worse over time?
* Are you losing in the:
  * opening
  * middle game
  * end game
* are there any repeated problems?
* intuitive feedback through visualisable data
* fun ways to learn openings??
  * assessment of your games opening win ratio
    * nice visual for win/loss


### steps
* collect data set
* store data set
* plug data into decision tree
  * any insight
* build interesting features
  * are my openings getting better, by opening
  * optimal 
    * time of day 
    * day of week


---

## Linfa tree documentation

https://www.freecodecamp.org/news/how-to-build-a-machine-learning-model-in-rust/

## Example linfa-tree usage

```rust
use linfa::traits::{Fit, Predict};
use linfa_trees::{DecisionTree, SplitCriterion};

// Age
// Gender (1 = male, 0 = female)
// Income
// Order history (number of orders placed)
// Website behavior (number of visits, in minutes)
// Email behavior (number of emails opened)
// Define the training data as a 2D array
let x = ndarray::arr2(&[
    [35.0, 1.0, 50000.0, 10.0, 120.0, 3.0],
    [45.0, 0.0, 75000.0, 20.0, 240.0, 2.0],
    [25.0, 1.0, 30000.0, 5.0, 60.0, 1.0],
    [30.0, 0.0, 40000.0, 15.0, 180.0, 4.0],
]);

// Define the target variable as a 1D array of binary labels
let y = ndarray::arr1(&[1.0, 1.0, 0.0, 1.0]);

// Create a new decision tree with a maximum depth of 2
let mut model = DecisionTree::params()
    .split_criterion(SplitCriterion::Gini)
    .max_depth(2)
    .fit(&x, &y)
    .unwrap();

// Use the model to make predictions on new data
let x_test = ndarray::arr2(&[
    [40.0, 1.0, 60000.0, 12.0, 90.0, 2.0],
    [28.0, 0.0, 35000.0, 8.0, 150.0, 3.0],
    [50.0, 1.0, 80000.0, 25.0, 300.0, 1.0],
]);
let y_pred = model.predict(&x_test);

// Print the predicted values
println!("{:?}", y_pred);
```