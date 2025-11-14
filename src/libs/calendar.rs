use chrono::{NaiveDate, Datelike, Duration, Weekday};
use anyhow::Result;

/// Calendar utilities
/// Equivalent to Go's libs/calendar.go

/// Get number of days in month
pub fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}

/// Check if year is leap year
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get all dates in a month
pub fn get_month_dates(year: i32, month: u32) -> Vec<NaiveDate> {
    let days = days_in_month(year, month);
    (1..=days)
        .filter_map(|day| NaiveDate::from_ymd_opt(year, month, day))
        .collect()
}

/// Get weekday name
pub fn weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

/// Get weekday name in Thai
pub fn weekday_name_th(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "จันทร์",
        Weekday::Tue => "อังคาร",
        Weekday::Wed => "พุธ",
        Weekday::Thu => "พฤหัสบดี",
        Weekday::Fri => "ศุกร์",
        Weekday::Sat => "เสาร์",
        Weekday::Sun => "อาทิตย์",
    }
}

/// Check if date is weekend
pub fn is_weekend(date: &NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Get next business day (skip weekends)
pub fn next_business_day(date: NaiveDate) -> NaiveDate {
    let mut next = date + Duration::days(1);
    while is_weekend(&next) {
        next = next + Duration::days(1);
    }
    next
}

/// Get previous business day (skip weekends)
pub fn previous_business_day(date: NaiveDate) -> NaiveDate {
    let mut prev = date - Duration::days(1);
    while is_weekend(&prev) {
        prev = prev - Duration::days(1);
    }
    prev
}

/// Count business days between two dates
pub fn count_business_days(start: NaiveDate, end: NaiveDate) -> i32 {
    let mut count = 0;
    let mut current = start;

    while current <= end {
        if !is_weekend(&current) {
            count += 1;
        }
        current = current + Duration::days(1);
    }

    count
}

/// Get date range
pub fn date_range(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut current = start;

    while current <= end {
        dates.push(current);
        current = current + Duration::days(1);
    }

    dates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 2), 29); // Leap year
        assert_eq!(days_in_month(2023, 2), 28); // Not leap year
        assert_eq!(days_in_month(2024, 1), 31);
        assert_eq!(days_in_month(2024, 4), 30);
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_is_weekend() {
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();

        assert!(is_weekend(&saturday));
        assert!(is_weekend(&sunday));
        assert!(!is_weekend(&monday));
    }

    #[test]
    fn test_count_business_days() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();   // Sunday

        assert_eq!(count_business_days(start, end), 5);
    }
}
