use std::collections::{BTreeMap, HashMap, HashSet};

fn is_special_sub_string(s1: &str, s2: &str) -> bool {
    let replaced_string = s2.replace(s1, "");
    replaced_string.len() < s2.len()
}

pub fn get_parent_child_strings(some_strings: Vec<String>) -> BTreeMap<String, Vec<String>> {
    let mut strings = some_strings.clone();
    strings.sort();

    let mut map = BTreeMap::new();

    for s in strings {
        let parts: Vec<_> = s.split("-").collect();
        let parent = parts[0..2].join("-");

        let child_vec = map.entry(parent.clone()).or_insert(Vec::new());

        let is_sub = is_special_sub_string(&parent, &s);
        if is_sub && !child_vec.contains(&s) {
            child_vec.push(s.clone());
            child_vec.sort();
        }
    }

    map
}

pub fn get_child_to_parent_map(some_strings: Vec<String>) -> HashMap<String, String> {
    let mut strings = some_strings.clone();
    strings.sort();

    let mut map = HashMap::new();

    for s in strings {
        let parts: Vec<_> = s.split("-").collect();
        let parent = parts[0..2].join("-");

        let is_sub = is_special_sub_string(&parent, &s);
        if is_sub {
            map.insert(s.clone(), parent.clone());
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::get_parent_child_strings;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn test_parent_child() {
        let openings = [
            "French-Defense-Tarrasch-Open-Delayed-Exchange-Variation",
            "French-Defense-Advance-Variation-3...c5-4.c3-Nc6",
            "French-Defense-Advance-Variation-3...c5-4.c3",
            "French-Defense-Tarrasch-Open-Euwe-Keres-Line-4...Nc6",
            "French-Defense-Exchange-Variation...4.Bd3-c5-5.dxc5-Bxc5",
            "French-Defense-Knight-Variation-2...d5-3.exd5-exd5-4.d4",
            "French-Defense-Exchange-Variation...4.Nf3-Nf6-5.Bd3-c5",
            "Queens-Gambit-Declined-Marshall-Defense...4.e4-Nf6-5.Nc3-e6",
            "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
            "Ruy-Lopez-Opening-Classical-Central-Variation",
            "Queens-Pawn-Opening-Accelerated-London-Steinitz-Countergambit",
            "Sicilian-Defense-French-Variation",
            "Modern-Defense-with-1-d4-2.c4-Bg7-3.Nc3",
            "Queens-Gambit-Declined-Queens-Knight-Variation-3...Nf6",
            "Kings-Pawn-Opening...2.Qe2-Nc6-3.c3-Nf6-4.Nf3",
            "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
            "Ruy-Lopez-Opening",
            "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
            "Scandinavian-Defense-Mieses-Kotrc-Gubinsky-Melts-Defense-4.Nf3",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = get_parent_child_strings(openings);
        assert_eq!(
            result,
            BTreeMap::from_iter(
                [
                    (
                        "French-Defense",
                        vec![
                            "French-Defense-Advance-Variation-3...c5-4.c3",
                            "French-Defense-Advance-Variation-3...c5-4.c3-Nc6",
                            "French-Defense-Exchange-Variation...4.Bd3-c5-5.dxc5-Bxc5",
                            "French-Defense-Exchange-Variation...4.Nf3-Nf6-5.Bd3-c5",
                            "French-Defense-Knight-Variation-2...d5-3.exd5-exd5-4.d4",
                            "French-Defense-Tarrasch-Open-Delayed-Exchange-Variation",
                            "French-Defense-Tarrasch-Open-Euwe-Keres-Line-4...Nc6",
                        ]
                    ),
                    (
                        "Kings-Pawn",
                        vec!["Kings-Pawn-Opening...2.Qe2-Nc6-3.c3-Nf6-4.Nf3"]
                    ),
                    (
                        "Modern-Defense",
                        vec!["Modern-Defense-with-1-d4-2.c4-Bg7-3.Nc3"]
                    ),
                    (
                        "Queens-Gambit",
                        vec![
                            "Queens-Gambit-Declined-Marshall-Defense...4.e4-Nf6-5.Nc3-e6",
                            "Queens-Gambit-Declined-Queens-Knight-Variation-3...Nf6"
                        ]
                    ),
                    (
                        "Queens-Pawn",
                        vec!["Queens-Pawn-Opening-Accelerated-London-Steinitz-Countergambit"]
                    ),
                    (
                        "Sicilian-Defense",
                        vec!["Sicilian-Defense-French-Variation"]
                    ),
                    (
                        "Scandinavian-Defense",
                        vec!["Scandinavian-Defense-Mieses-Kotrc-Gubinsky-Melts-Defense-4.Nf3"]
                    ),
                    (
                        "Ruy-Lopez",
                        vec![
                            "Ruy-Lopez-Opening",
                            "Ruy-Lopez-Opening-Classical-Central-Variation",
                            "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
                        ]
                    ),
                ]
                .iter()
                .map(|(k, v)| (
                    k.to_string(),
                    v.iter().map(|s| s.to_string()).collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>()
            )
        );
    }

    #[test]
    fn test_parent_child_small() {
        let openings = [
            "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
            "Ruy-Lopez-Opening",
            "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = get_parent_child_strings(openings);
        assert_eq!(
            result,
            BTreeMap::from_iter(
                [(
                    "Ruy-Lopez",
                    vec![
                        "Ruy-Lopez-Opening",
                        "Ruy-Lopez-Opening-Cozio-Defense-4.c3-a6-5.Ba4",
                    ]
                ),]
                .iter()
                .map(|(k, v)| (
                    k.to_string(),
                    v.iter().map(|s| s.to_string()).collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>()
            )
        );
    }
}
