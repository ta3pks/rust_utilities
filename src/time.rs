use std::time::UNIX_EPOCH;
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

impl_for!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f64, f32);

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
pub trait SleepExt {
    fn sleep_sync(&self);
}
impl SleepExt for std::time::Duration {
    fn sleep_sync(&self) {
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

pub fn now_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
pub fn now_micros() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_micros() as u64
}
pub fn now_secs() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_secs()
}
pub fn now_nanos() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_nanos() as u64
}
