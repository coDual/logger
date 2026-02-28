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
            Self::Weekly => "w",
            Self::Monthly => "m",
            Self::Bimonthly => "b",
        }
    }

    fn number(&self, date: &impl Datelike) -> u32 {
        match self {
            Self::Weekly => date.iso_week().week(),
            Self::Monthly => date.month(),
            Self::Bimonthly => date.month() / 2,
        }
    }

    pub fn filename(&self, date: &impl Datelike) -> String {
        format!("{}{:02}.md", self.prefix(), self.number(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_filename() {
        let date = NaiveDate::from_ymd_opt(2021, 4, 24).unwrap();
        assert_eq!(Frequency::Weekly.filename(&date), "w16.md");
        assert_eq!(Frequency::Monthly.filename(&date), "m04.md");
        assert_eq!(Frequency::Bimonthly.filename(&date), "b02.md");
    }
}
