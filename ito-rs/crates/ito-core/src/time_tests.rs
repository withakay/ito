use chrono::{NaiveDate, NaiveTime};

use super::{now_date, now_time};

#[test]
fn clock_helpers_return_the_documented_formats() {
    NaiveTime::parse_from_str(&now_time(), "%H:%M:%S")
        .expect("current time should use the documented format");
    NaiveDate::parse_from_str(&now_date(), "%Y-%m-%d")
        .expect("current date should use the documented format");
}
