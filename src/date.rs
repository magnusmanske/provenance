use chrono::{Datelike, NaiveDate};
use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Date {
    Day(NaiveDate),
    Month { month: u8, year: i32 },
    Year(i32),
}

impl Date {
    fn as_naive_date(&self) -> Option<NaiveDate> {
        match self {
            Date::Day(date) => Some(*date),
            Date::Month { month, year } => NaiveDate::from_ymd_opt(*year, *month as u32, 1),
            Date::Year(year) => NaiveDate::from_ymd_opt(*year, 1, 1),
        }
    }

    /// Parse a date from a string. The year must be the last text in the string.
    pub fn from_text(text: &str) -> Option<Date> {
        lazy_static! {
            pub static ref RE_DAY_MONTH_YEAR: Regex =
                Regex::new(r"(\d{1,2}\.? [A-Z]\S+ \d{3,})$").unwrap();
            pub static ref RE_MONTH_YEAR: Regex = Regex::new(r"\b([A-Z]\S+ \d{3,})$").unwrap();
            pub static ref RE_YEAR: Regex = Regex::new(r"\b(\d{3,})$").unwrap();
        }

        if let Some(captures) = RE_DAY_MONTH_YEAR.captures(text) {
            return NaiveDate::parse_from_str(&captures[1], "%d %B %Y")
                .ok()
                .map(Date::Day);
        }

        if let Some(captures) = RE_MONTH_YEAR.captures(text) {
            let fake_day = format!("1 {}", &captures[1]);
            if let Ok(date) = NaiveDate::parse_from_str(&fake_day, "%d %B %Y") {
                return Some(Date::Month {
                    month: date.month() as u8,
                    year: date.year(),
                });
            }
        }
        if let Some(captures) = RE_YEAR.captures(text) {
            return captures[1].parse().ok().map(Date::Year);
        }
        None
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Date::Day(date) => write!(f, "{}", date),
            Date::Month { month, year } => write!(f, "{}/{}", month, year),
            Date::Year(year) => write!(f, "{}", year),
        }
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_naive_date().cmp(&other.as_naive_date())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_from_text() {
        assert_eq!(
            Date::from_text("12 January 2020"),
            Some(Date::Day(NaiveDate::from_ymd_opt(2020, 1, 12).unwrap()))
        );
        assert_eq!(
            Date::from_text("January 2020"),
            Some(Date::Month {
                month: 1,
                year: 2020
            })
        );
        assert_eq!(Date::from_text("2020"), Some(Date::Year(2020)));
    }

    #[test]
    fn test_date_display() {
        assert_eq!(
            format!(
                "{}",
                Date::Day(NaiveDate::from_ymd_opt(2020, 1, 12).unwrap())
            ),
            "2020-01-12"
        );
        assert_eq!(
            format!(
                "{}",
                Date::Month {
                    month: 1,
                    year: 2020
                }
            ),
            "1/2020"
        );
        assert_eq!(format!("{}", Date::Year(2020)), "2020");
    }
}
