use chrono::{DateTime, Utc, TimeZone, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let end = start.timestamp()+1000000000;
    Utc.timestamp(end, 0)
}

// alternative solution
pub fn after_refectored(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}

