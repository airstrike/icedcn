//! Month geometry for Gregorian and Solar Hijri calendars.

use chrono::{Datelike, Duration, Local};
use jalali_rs::{gregorian_to_jalali, jalali_to_gregorian, latin_digits_to_persian};

pub use chrono::Weekday;
pub type Date = chrono::NaiveDate;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum System {
    #[default]
    Gregorian,
    Persian,
}

pub fn today() -> Date {
    Local::now().date_naive()
}

impl System {
    pub fn parts(self, date: Date) -> (i32, u32, u32) {
        match self {
            Self::Gregorian => (date.year(), date.month(), date.day()),
            Self::Persian => {
                gregorian_to_jalali(date.year(), date.month() as usize, date.day() as i32)
            }
        }
    }

    pub fn date(self, year: i32, month: u32, day: u32) -> Option<Date> {
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return None;
        }
        let (gy, gm, gd) = match self {
            Self::Gregorian => (year, month, day),
            Self::Persian => jalali_to_gregorian(year, month as usize, day as i32),
        };
        Date::from_ymd_opt(gy, gm, gd).filter(|date| self.parts(*date) == (year, month, day))
    }

    pub(crate) fn first(self, date: Date) -> Date {
        let (_, _, day) = self.parts(date);
        date.checked_sub_signed(Duration::days(i64::from(day) - 1))
            .unwrap_or(date)
    }

    pub fn shift(self, date: Date, months: i32) -> Option<Date> {
        let (year, month, _) = self.parts(date);
        let ordinal = year
            .checked_mul(12)?
            .checked_add(month as i32 - 1)?
            .checked_add(months)?;
        self.date(ordinal.div_euclid(12), ordinal.rem_euclid(12) as u32 + 1, 1)
    }

    pub fn month_name(self, month: u32) -> &'static str {
        let names = match self {
            Self::Gregorian => [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ],
            Self::Persian => [
                "فروردین",
                "اردیبهشت",
                "خرداد",
                "تیر",
                "مرداد",
                "شهریور",
                "مهر",
                "آبان",
                "آذر",
                "دی",
                "بهمن",
                "اسفند",
            ],
        };
        month
            .checked_sub(1)
            .and_then(|index| names.get(index as usize))
            .copied()
            .unwrap_or("")
    }

    pub(crate) fn number(self, number: impl ToString) -> String {
        let s = number.to_string();
        match self {
            Self::Gregorian => s,
            Self::Persian => latin_digits_to_persian(&s),
        }
    }

    pub(crate) fn weeks(self, month: Date) -> Vec<[Date; 7]> {
        let first = self.first(month);
        let end = self.shift(first, 1).unwrap_or(Date::MAX);
        let weekday = first.weekday().num_days_from_sunday();
        let offset = match self {
            Self::Gregorian => weekday,
            Self::Persian => (weekday + 1) % 7,
        };
        let Some(mut start) = first.checked_sub_signed(Duration::days(i64::from(offset))) else {
            return Vec::new();
        };
        let mut weeks = vec![];
        while start < end {
            if start.checked_add_signed(Duration::days(6)).is_none() {
                break;
            }
            weeks.push(std::array::from_fn(|i| start + Duration::days(i as i64)));
            let Some(next) = start.checked_add_signed(Duration::weeks(1)) else {
                break;
            };
            start = next;
        }
        weeks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn month_grids_start_on_the_correct_weekday_and_include_leap_days() {
        let feb = Date::from_ymd_opt(2024, 2, 1).unwrap();
        let weeks = System::Gregorian.weeks(feb);
        assert_eq!(weeks[0][0].weekday(), Weekday::Sun);
        assert!(
            weeks
                .iter()
                .flatten()
                .any(|d| d.month() == 2 && d.day() == 29)
        );
        let nowruz = Date::from_ymd_opt(2025, 3, 21).unwrap();
        assert_eq!(System::Persian.parts(nowruz), (1404, 1, 1));
        assert_eq!(System::Persian.weeks(nowruz)[0][0].weekday(), Weekday::Sat);
        assert_eq!(System::Persian.date(1404, 1, 1), Some(nowruz));
    }
}
