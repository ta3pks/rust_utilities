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
            fn minutes(&self) -> std::time::Duration {
                std::time::Duration::from_secs(self.seconds().as_secs() * 60)
            }
            fn hours(&self) -> std::time::Duration {
                std::time::Duration::from_secs(self.minutes().as_secs() * 60)
            }
        }
    )*
    };
}

pub trait DurationExt {
    fn millis(&self) -> std::time::Duration;
    fn seconds(&self) -> std::time::Duration;
    fn minutes(&self) -> std::time::Duration;
    fn hours(&self) -> std::time::Duration;
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
