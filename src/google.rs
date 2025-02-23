#[cfg(feature = "listings")]
pub mod r#type {
    include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));

    #[cfg(feature = "rust_decimal")]
    impl From<&Money> for rust_decimal::Decimal {
        fn from(value: &Money) -> Self {
            let decimal_units = Self::new(value.units, 0);
            let nanos_as_decimal = Self::new(value.nanos as i64, 9);
            decimal_units + nanos_as_decimal
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[cfg(feature = "rust_decimal")]
        use rust_decimal::Decimal;

        #[test]
        #[cfg(feature = "rust_decimal")]
        fn test_positive_amount() {
            let money = Money {
                currency_code: "USD".to_string(),
                units: 2,
                nanos: 500_000_000,
            };

            let decimal = Decimal::from(&money);
            assert_eq!(decimal, Decimal::new(25, 1)); // 2.5
        }

        #[test]
        #[cfg(feature = "rust_decimal")]
        fn test_negative_amount() {
            let money = Money {
                currency_code: "USD".to_string(),
                units: -1,
                nanos: -750_000_000,
            };

            let decimal = Decimal::from(&money);
            assert_eq!(decimal, Decimal::new(-1750000000, 9)); // -1.750000000
        }

        #[test]
        #[cfg(feature = "rust_decimal")]
        fn test_zero_amount() {
            let money = Money {
                currency_code: "USD".to_string(),
                units: 0,
                nanos: 0,
            };

            let decimal = Decimal::from(&money);
            assert_eq!(decimal, Decimal::new(0, 0)); // 0.0
        }

        #[test]
        #[cfg(feature = "rust_decimal")]
        fn test_large_nanos() {
            let money = Money {
                currency_code: "USD".to_string(),
                units: 3,
                nanos: 999_999_999,
            };

            let decimal = Decimal::from(&money);
            assert_eq!(decimal, Decimal::new(30, 1) + Decimal::new(999_999_999, 9));
            // 3.999999999
        }

        #[test]
        #[cfg(feature = "rust_decimal")]
        fn test_large_negative_nanos() {
            let money = Money {
                currency_code: "USD".to_string(),
                units: -4,
                nanos: -999_999_999,
            };

            let decimal = Decimal::from(&money);
            assert_eq!(
                decimal,
                Decimal::new(-40, 1) + Decimal::new(-999_999_999, 9)
            ); // -4.999999999
        }

        #[test]
        #[cfg(feature = "rust_decimal")]
        fn test_mixed_case_with_large_and_small_units() {
            let money = Money {
                currency_code: "USD".to_string(),
                units: 1,
                nanos: 999_999_500,
            };

            let decimal = Decimal::from(&money);
            assert_eq!(decimal, Decimal::new(1, 0) + Decimal::new(999_999_500, 9));
            // 1.999999500
        }
    }
}

pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

    #[cfg(feature = "time")]
    impl From<time::OffsetDateTime> for Timestamp {
        fn from(dt: time::OffsetDateTime) -> Self {
            Timestamp {
                seconds: dt.unix_timestamp(),
                nanos: dt.nanosecond() as i32,
            }
        }
    }

    #[cfg(feature = "time")]
    impl TryFrom<Timestamp> for time::OffsetDateTime {
        type Error = time::Error;

        fn try_from(timestamp: Timestamp) -> Result<Self, Self::Error> {
            let seconds = timestamp.seconds;
            let nanos = timestamp.nanos as i64;
            let nanoseconds = nanos % 1_000_000_000;
            let d = time::OffsetDateTime::from_unix_timestamp(seconds)?
                + time::Duration::nanoseconds(nanoseconds);
            Ok(d)
        }
    }

    #[cfg(test)]
    #[cfg(feature = "time")]
    mod tests {
        use super::*;
        use time::{Duration, OffsetDateTime};

        #[test]
        fn test_offsetdatetime_to_timestamp() {
            let now = OffsetDateTime::now_utc();
            let timestamp: Timestamp = now.into();

            assert_eq!(timestamp.seconds, now.unix_timestamp());
            assert_eq!(timestamp.nanos, now.nanosecond() as i32);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime() {
            let now = OffsetDateTime::now_utc();
            let timestamp: Timestamp = now.into();
            let dt: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt, now);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_with_nanos() {
            let now = OffsetDateTime::now_utc();
            let nanos = 123456789;
            let dt = now + Duration::nanoseconds(nanos);
            let timestamp: Timestamp = dt.into();
            let dt_from_timestamp: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt_from_timestamp, dt);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_with_negative_nanos() {
            let now = OffsetDateTime::now_utc();
            let nanos = -123456789;
            let dt = now + Duration::nanoseconds(nanos);
            let timestamp: Timestamp = dt.into();
            let dt_from_timestamp: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt_from_timestamp, dt);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_invalid_seconds() {
            let timestamp = Timestamp {
                seconds: i64::MIN,
                nanos: 0,
            };
            let result: Result<OffsetDateTime, time::Error> = timestamp.try_into();
            assert!(result.is_err());
        }
    }
}
