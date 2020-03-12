#![allow(unused)]

use chrono::{Duration, NaiveDate, NaiveDateTime};

pub trait VerticaDate {
    fn to_y2k_epoch_duration(&self) -> Duration;
}

impl VerticaDate for NaiveDate {
    fn to_y2k_epoch_duration(&self) -> Duration {
        *self - NaiveDate::from_ymd(2000, 1, 1)
    }
}

impl VerticaDate for NaiveDateTime {
    fn to_y2k_epoch_duration(&self) -> Duration {
        *self - NaiveDate::from_ymd(2000, 1, 1).and_hms_micro(0, 0, 0, 0)
    }
}

fn _seconds_since_midnight(hours: i32, minutes: i32, seconds: i32) -> i32 {
    3600 * hours + 60 * minutes + seconds
}

fn _microseconds_since_midnight(hours: i32, minutes: i32, seconds: i32) -> u64 {
    1_000_000u64 * _seconds_since_midnight(hours, minutes, seconds) as u64
}

pub fn _timetz(hours: i32, minutes: i32, seconds: i32, timezone: i32) -> u64 {
    ((_microseconds_since_midnight(hours - timezone, minutes, seconds) << 24)
        | _seconds_since_midnight(24 - timezone, 0, 0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertica_epoch_days() {
        assert_eq!(
            -358,
            NaiveDate::from_ymd(1999, 1, 8)
                .to_y2k_epoch_duration()
                .num_days()
        );
        assert_eq!(
            0,
            NaiveDate::from_ymd(2000, 1, 1)
                .to_y2k_epoch_duration()
                .num_days()
        );
        assert_eq!(
            366,
            NaiveDate::from_ymd(2001, 1, 1)
                .to_y2k_epoch_duration()
                .num_days()
        );
    }

    #[test]
    fn test_seconds_since_midnight() {
        assert_eq!(
            [0xd0u8, 0x97, 0x1, 0x0],
            _seconds_since_midnight(29, 0, 0).to_le_bytes()
        )
    }

    #[test]
    fn test_microseconds_since_midnight() {
        assert_eq!(1_000_000u64, _microseconds_since_midnight(0, 0, 1));
        assert_eq!(61_000_000u64, _microseconds_since_midnight(0, 1, 1));
        assert_eq!(3661_000_000u64, _microseconds_since_midnight(1, 1, 1));
        assert_eq!(
            [0x80u8, 0xf0, 0x79, 0xf0, 0x10, 0, 0, 0],
            _microseconds_since_midnight(20, 12, 34).to_le_bytes()
        )
    }

    #[test]
    fn test_timetz() {
        // TIMETZ - 15:12:34-05
        assert_eq!(
            [0xd0u8, 0x97, 0x01, 0x80, 0xf0, 0x79, 0xf0, 0x10],
            _timetz(15, 12, 34, -5).to_le_bytes()
        )
    }
}
