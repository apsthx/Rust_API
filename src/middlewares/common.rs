use chrono::{NaiveDate, NaiveDateTime};
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

/// Convert string to integer
/// Equivalent to Go's StrToInt in middlewares/common.go
pub fn str_to_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

/// Convert string to float
/// Equivalent to Go's StrToFloat
pub fn str_to_float(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.parse::<f64>()
}

/// Hash password using bcrypt
/// Equivalent to Go's hashPassword function
pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| anyhow::anyhow!(e))
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash).map_err(|e| anyhow::anyhow!(e))
}

/// Parse date string to NaiveDate
/// Equivalent to Go's date parsing functions
pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!(e))
}

/// Parse datetime string to NaiveDateTime
pub fn parse_datetime(datetime_str: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| anyhow::anyhow!(e))
}

/// Compare two dates
pub fn is_date_before(date1: &NaiveDate, date2: &NaiveDate) -> bool {
    date1 < date2
}

/// Compare two dates
pub fn is_date_after(date1: &NaiveDate, date2: &NaiveDate) -> bool {
    date1 > date2
}

/// Check if date is between two dates (inclusive)
pub fn is_date_between(date: &NaiveDate, start: &NaiveDate, end: &NaiveDate) -> bool {
    date >= start && date <= end
}

/// Get distinct values from vector
/// Equivalent to Go's DistinctArr
pub fn distinct_vec<T: Clone + Eq + std::hash::Hash>(vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

/// Get difference between two vectors
/// Equivalent to Go's DifferenceArr
pub fn difference_vec<T: Clone + Eq + std::hash::Hash>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;
    let set2: HashSet<_> = vec2.into_iter().collect();
    vec1.into_iter().filter(|item| !set2.contains(item)).collect()
}

/// Generate random string
/// Equivalent to Go's random string generation
pub fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate random number between min and max
pub fn generate_random_number(min: i32, max: i32) -> i32 {
    thread_rng().gen_range(min..=max)
}

/// Format float to string with decimal places
pub fn format_float(value: f64, decimal_places: usize) -> String {
    format!("{:.prec$}", value, prec = decimal_places)
}

/// Check if string is empty or whitespace
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

/// Trim and lowercase string
pub fn normalize_string(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Convert Option<String> to String with default
pub fn option_string_or_default(opt: Option<String>, default: &str) -> String {
    opt.unwrap_or_else(|| default.to_string())
}

/// Convert Option<i32> to i32 with default
pub fn option_i32_or_default(opt: Option<i32>, default: i32) -> i32 {
    opt.unwrap_or(default)
}

/// Convert Vec to comma-separated string
pub fn vec_to_csv<T: ToString>(vec: Vec<T>) -> String {
    vec.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// Convert comma-separated string to Vec<i32>
pub fn csv_to_vec_i32(csv: &str) -> Vec<i32> {
    csv.split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}

/// Calculate percentage
pub fn calculate_percentage(value: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (value / total) * 100.0
    }
}

/// Round float to n decimal places
pub fn round_to_decimal(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10_f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

/// Check if value is in range
pub fn is_in_range<T: PartialOrd>(value: T, min: T, max: T) -> bool {
    value >= min && value <= max
}

/// Clamp value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_int() {
        assert_eq!(str_to_int("123").unwrap(), 123);
        assert!(str_to_int("abc").is_err());
    }

    #[test]
    fn test_str_to_float() {
        assert_eq!(str_to_float("123.45").unwrap(), 123.45);
        assert!(str_to_float("abc").is_err());
    }

    #[test]
    fn test_hash_and_verify_password() {
        let password = "test_password";
        let hashed = hash_password(password).unwrap();
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn test_distinct_vec() {
        let vec = vec![1, 2, 2, 3, 3, 3, 4];
        let distinct = distinct_vec(vec);
        assert_eq!(distinct.len(), 4);
    }

    #[test]
    fn test_difference_vec() {
        let vec1 = vec![1, 2, 3, 4, 5];
        let vec2 = vec![3, 4, 5, 6, 7];
        let diff = difference_vec(vec1, vec2);
        assert_eq!(diff, vec![1, 2]);
    }

    #[test]
    fn test_generate_random_string() {
        let s = generate_random_string(10);
        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(25.0, 100.0), 25.0);
        assert_eq!(calculate_percentage(50.0, 0.0), 0.0);
    }

    #[test]
    fn test_round_to_decimal() {
        assert_eq!(round_to_decimal(3.14159, 2), 3.14);
        assert_eq!(round_to_decimal(3.14159, 4), 3.1416);
    }

    #[test]
    fn test_is_in_range() {
        assert!(is_in_range(5, 1, 10));
        assert!(!is_in_range(11, 1, 10));
    }
}
