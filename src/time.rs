use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(feature = "async")]
use std::{future::Future, pin::Pin};

macro_rules! impl_for{
    ($($t:ty),*) => {
        $(
        impl DurationExt for $t {
            fn millis(&self) -> std::time::Duration {
                std::time::Duration::from_millis(*self as u64)
            }
            fn seconds(&self) -> std::time::Duration {
                std::time::Duration::from_secs(*self as u64)
            }
        }
    )*
    };
}

pub trait DurationExt {
    fn millis(&self) -> std::time::Duration;
    fn seconds(&self) -> std::time::Duration;
    fn minutes(&self) -> std::time::Duration {
        self.seconds() * 60
    }
    fn hours(&self) -> std::time::Duration {
        self.minutes() * 60
    }
    fn days(&self) -> std::time::Duration {
        self.hours() * 24
    }
}

impl_for!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f64, f32, u128, i128);

#[cfg(feature = "async")]
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
#[cfg(feature = "time_async")]
pub trait SleepAsyncExt {
    fn sleep_async(&self) -> BoxFuture<'_, ()>;
}
#[cfg(feature = "time_async")]
impl SleepAsyncExt for std::time::Duration {
    fn sleep_async(&self) -> BoxFuture<'_, ()> {
        Box::pin(tokio::time::sleep(*self))
    }
}

pub trait TimeExt {
    fn unix_secs(&self) -> u64;
    fn unix_millis(&self) -> u64;
    fn unix_nanos(&self) -> u64;
    fn unix_micros(&self) -> u64;
    fn unix_days(&self) -> u32 {
        (self.unix_secs() / (60 * 60 * 24)) as u32
    }
}
impl TimeExt for SystemTime {
    fn unix_millis(&self) -> u64 {
        self.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    fn unix_secs(&self) -> u64 {
        self.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    fn unix_nanos(&self) -> u64 {
        self.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }

    fn unix_micros(&self) -> u64 {
        self.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64
    }
}

pub trait SleepExt {
    fn sleep_sync(&self);
}
impl SleepExt for std::time::Duration {
    fn sleep_sync(&self) {
        SystemTime::now().unix_secs();
        std::thread::sleep(*self)
    }
}
pub trait HMS {
    fn hms(&self) -> String;
    fn hmsxxx(&self) -> String;
}

impl HMS for std::time::Duration {
    fn hms(&self) -> String {
        let secs = self.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    fn hmsxxx(&self) -> String {
        let secs = self.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        let millis = self.subsec_millis();
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    }
}
pub fn now() -> SystemTime {
    SystemTime::now()
}
pub fn instant_now() -> std::time::Instant {
    std::time::Instant::now()
}
pub trait InstantExt {
    fn print_elapsed(&self);
    fn reset(&mut self);
    fn eprint_elapsed(&self);
}
impl InstantExt for std::time::Instant {
    fn print_elapsed(&self) {
        let elapsed = self.elapsed();
        println!("{elapsed:?}");
    }
    fn eprint_elapsed(&self) {
        let elapsed = self.elapsed();
        eprintln!("{elapsed:?}");
    }
    fn reset(&mut self) {
        *self = std::time::Instant::now();
    }
}
impl InstantExt for std::time::SystemTime {
    fn print_elapsed(&self) {
        let elapsed = self.elapsed().unwrap_or_default();
        println!("{elapsed:?}");
    }
    fn eprint_elapsed(&self) {
        let elapsed = self.elapsed().unwrap_or_default();
        eprintln!("{elapsed:?}");
    }
    fn reset(&mut self) {
        *self = std::time::SystemTime::now();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hms() {
        let d = 1.hours() + 1.minutes() + 1.seconds() + 10.millis();
        assert_eq!(d.hms(), "01:01:01");
        assert_eq!(d.hmsxxx(), "01:01:01.010");
    }
}
