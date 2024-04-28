use chrono::Datelike;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Weekly,
    Monthly,
    Bimonthly,
}

impl Frequency {
    fn prefix(&self) -> &'static str {
        match self {
            Frequency::Weekly => "w",
            Frequency::Monthly => "m",
            Frequency::Bimonthly => "b",
        }
    }
}

pub fn get_filename(frequency: &Frequency, date: impl Datelike) -> String {
    let number = match frequency {
        Frequency::Weekly => date.iso_week().week(),
        Frequency::Monthly => date.month(),
        Frequency::Bimonthly => date.month() / 2,
    };
    format!("{}{:02}.md", frequency.prefix(), number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    #[test]
    fn test_get_filename() {
        let date = NaiveDate::from_ymd_opt(2021, 4, 24).unwrap();
        assert_eq!(get_filename(&Frequency::Weekly, date), "w16.md");
        assert_eq!(get_filename(&Frequency::Monthly, date), "m04.md");
        assert_eq!(get_filename(&Frequency::Bimonthly, date), "b02.md");
    }
}
