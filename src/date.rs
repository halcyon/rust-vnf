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
}
