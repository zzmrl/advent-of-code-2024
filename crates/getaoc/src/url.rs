use url::{ParseError, Url};

const BASE_URL: &str = "https://adventofcode.com/";

pub fn base_url() -> Result<Url, ParseError> {
    BASE_URL.parse::<Url>()
}

pub fn build_input_url(year: u16, day: u8) -> Result<Url, ParseError> {
    let base_url = base_url()?;
    let path = format!("{year}/day/{day}/input");
    base_url.join(path.as_str())
}
