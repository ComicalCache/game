mod bar;
pub use bar::Bar;

mod ratio_bar;
pub use ratio_bar::RatioBar;

#[cfg(feature = "cmdln")]
pub mod config {
    pub const BAR_WIDTH: usize = 55;

    pub const BAR_BRACKET_LEFT: &str = "[";
    pub const BAR_BRACKET_RIGHT: &str = "]";
    pub const BAR_FILL: char = '#';
    pub const BAR_BLANK: char = ' ';
    pub const RATIO_BAR_SEPARATOR: char = '|';
}

#[cfg(feature = "cmdln")]
pub fn thousands_separator<T: Into<u64>>(num: T) -> String {
    let num: u64 = num.into();
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

#[cfg(feature = "cmdln")]
pub fn shorten_uuid(uuid: uuid::Uuid) -> String {
    const BRACKET_LEFT: &str = "[";
    const BRACKET_RIGHT: &str = "]";

    let uuid_string = uuid.to_string();

    format!(
        "{BRACKET_LEFT}{}{BRACKET_RIGHT}",
        uuid_string
            .chars()
            .take(6)
            .chain("...".chars())
            .chain(uuid_string.chars().skip(30))
            .collect::<String>()
    )
}
