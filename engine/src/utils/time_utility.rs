use chrono::Utc;

pub fn get_current_timestamp() -> i64 {
    Utc::now().timestamp()
}
