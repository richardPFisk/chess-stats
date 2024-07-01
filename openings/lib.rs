use std::io;
pub mod models;

use std::fs::File;
use std::io::Read;

use csv::ReaderBuilder;
static OPENING: &str = include_str!("a.tsv");

pub fn read_file(name: &'static str) -> io::Result<String> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.pop(); // remove trailing newline
    Ok(contents)
}

pub fn parse_tsv_files() -> Result<bool, io::Error> {
    let tab_ch = r#"	"#;
    let mut rdr = ReaderBuilder::new()
        .delimiter(*tab_ch.as_bytes().get(0).unwrap())
        .from_path("./a.tsv")?;

    if let Some(result) = rdr.records().next() {
        println!("{result:#?}");
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse_tsv_files();
        assert_eq!(result.unwrap_or(false), true);
    }
}
