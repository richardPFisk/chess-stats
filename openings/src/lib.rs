use std::io;
pub mod models;

use csv::ReaderBuilder;
use models::Opening;

pub fn read_file(name: &'static str) -> Result<Vec<Opening>, io::Error> {
    let tab_ch = r#"	"#.as_bytes().get(0).unwrap();
    let rdr = ReaderBuilder::new()
        .delimiter(*tab_ch)
        .from_path(name)?;

    let openings = rdr.into_deserialize()
        .collect::<Result<Vec<Opening>, csv::Error>>()?;

    Ok(openings)
}

pub fn parse_tsv_files() -> Result<Vec<Opening>, io::Error> {
    let a_openings = read_file("./a.tsv")?;
    let b_openings = read_file("./b.tsv")?;
    let c_openings = read_file("./c.tsv")?;
    let d_openings = read_file("./d.tsv")?;
    let e_openings = read_file("./e.tsv")?;

    Ok(vec![a_openings, b_openings, c_openings, d_openings, e_openings].concat())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chess_pgn::models::headers::PgnData;
    use models::ECO;

    use super::*;

    #[test]
    fn it_works() {
        let result = parse_tsv_files();
        assert_eq!(result.unwrap_or(vec![]).len(), 3469);
    }

    #[test]
    fn it_has_data() {
        let result = parse_tsv_files();
        let opening = Opening { eco: ECO("A00".to_owned()), name: "Fred".to_owned(), pgn: PgnData { headers: HashMap::new(), moves: vec![] } };
        let fake_opening = Some(&opening);
        assert_eq!(result.unwrap_or(vec![]).last(), fake_opening);
    }
}
