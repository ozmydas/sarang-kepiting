use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Asia::Jakarta;

pub fn today_jakarta() -> String {
    let now_utc: DateTime<Utc> = Utc::now();
    let now_jakarta = now_utc.with_timezone(&Jakarta);
    let today_jakarta = now_jakarta.date_naive();

    today_jakarta.to_string()
} // end func

pub fn date_diff(start_date: String, end_date: String) -> i64 {
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").expect("Invalid date format");

    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").expect("Invalid date format");

    let difference = end.signed_duration_since(start).num_days();

    difference
} // end func
