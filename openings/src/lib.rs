use std::io;

pub mod models;
pub mod trie;

use csv::ReaderBuilder;
use models::{OpeningClassification, Opening};

pub fn read_file(name: &'static str) -> Result<Vec<Opening>, io::Error> {
    let tab_ch = r#"	"#.as_bytes().first().unwrap();
    let rdr = ReaderBuilder::new().delimiter(*tab_ch).from_path(name)?;

    let openings = rdr
        .into_deserialize()
        .collect::<Result<Vec<Opening>, csv::Error>>()?;

    Ok(openings)
}

pub fn parse_tsv_files() -> Result<Vec<Opening>, io::Error> {
    let a_openings = read_file("./a.tsv")?;
    let b_openings = read_file("./b.tsv")?;
    let c_openings = read_file("./c.tsv")?;
    let d_openings = read_file("./d.tsv")?;
    let e_openings = read_file("./e.tsv")?;

    Ok([a_openings, b_openings, c_openings, d_openings, e_openings].concat())
}


pub fn filter_by_opening_family(opening_filter: OpeningClassification, openings: Vec<Opening>) -> Vec<Opening> {
    openings.into_iter().filter(|o|{ o.name.family == opening_filter.family }).collect()
}

pub fn get_families(openings: Vec<Opening>) -> Vec<Opening> {
    openings.into_iter().filter(|o|{ o.name.sub_variation.is_none() && o.name.variation.is_none() }).collect()
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chess_pgn::models::headers::PgnData;
    use models::{OpeningClassification, ECO};

    use super::*;

    #[test]
    fn it_works() {
        let result = parse_tsv_files();
        assert_eq!(result.unwrap_or_default().len(), 3469);
    }

    #[test]
    fn it_has_sicialian_data() {
        let result = parse_tsv_files();
        
        let opening_name = OpeningClassification {
            family: "Sicilian Defense".to_owned(),
            variation: None,
            sub_variation: None,
        };
        let opening = Opening {
            eco: ECO("B20".to_owned()),
            name: opening_name.to_owned(),
            pgn: PgnData {
                headers: HashMap::new(),
                moves: vec!["e4".to_owned(), "c5".to_owned()],
            },
        };
        let unwrapped_result = result.unwrap();
        let filtered_result = filter_by_opening_family(opening_name, unwrapped_result);
        let count = filtered_result.iter().cloned().len();
        
        assert_eq!(count, 370);
        assert_eq!(filtered_result.first().unwrap(), &opening);
    }

    #[test]
    fn test_get_families() {
        let result = parse_tsv_files();
        let unwrapped_result = result.unwrap();
        let filtered_result = get_families(unwrapped_result);
        let count = filtered_result.iter().cloned().len();
        
        assert_eq!(count, 192);
        assert_eq!(filtered_result.first().unwrap().name,  OpeningClassification {
            family: "Amar Opening".to_owned(),
            variation: None,
            sub_variation: None,
        });
        assert_eq!(filtered_result.last().unwrap().name,  OpeningClassification {
            family: "King's Indian Defense".to_owned(),
            variation: None,
            sub_variation: None,
        });
    }
}
