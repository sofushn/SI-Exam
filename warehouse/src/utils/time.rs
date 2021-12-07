use crate::entities::person;
use chrono::prelude::*;

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_current_time_and_add_request_millis(
    request: &person::GenerateCodeAndStartRequest,
) -> NaiveDateTime {
    let utc: DateTime<Utc> = Utc::now();
    let secs = utc.timestamp() + parse_request_time(request.date_time.as_str()).timestamp();
    NaiveDateTime::from_timestamp(secs, 0)
}

pub fn parse_request_time(timestamp_string: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(timestamp_string, TIME_FORMAT)
        .expect("Parse naive date time failed")
}

// Literally just checks which datetime is the latest
pub fn time_expired(verification_time: NaiveDateTime) -> bool {
    Utc::now().timestamp() > verification_time.timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_request_time_returns_correct_day() {
        let date_time = parse_request_time("1970-03-12 12:32:19");
        assert_eq!(date_time.day(), 12);
    }

    #[test]
    fn time_expired_returns_true() {
        assert_eq!(
            time_expired(parse_request_time("1970-03-12 12:32:19")),
            true
        );
    }

    #[test]
    fn time_expired_returns_false() {
        assert_eq!(
            time_expired(parse_request_time("2070-03-12 12:32:19")),
            false
        );
    }
}
