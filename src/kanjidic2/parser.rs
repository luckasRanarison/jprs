use quick_xml::de::from_str;
use std::fs::read_to_string;

use crate::{kanjidic2::types::Kanjidic2, Error, Result};

pub fn parse(file_path: &str) -> Result<Kanjidic2> {
    let source = read_to_string(file_path)?;
    let start = source.find("<kanjidic2>").ok_or(Error::InvalidFile)?;
    let parsed = from_str(&source[start..])?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parsing() {
        assert!(parse("data/kanjidic2.xml").is_ok());
    }
}
