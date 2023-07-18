macro_rules! impl_for{
    ($($t:ty),*) => {
        $(
        impl DurationExt for $t {
            fn millis(&self) -> std::time::Duration {
                (*self as u64).millis()
            }
            fn seconds(&self) -> std::time::Duration {
                (*self as u64).seconds()
            }
            fn minutes(&self) -> std::time::Duration {
                (*self as u64).minutes()
            }
            fn hours(&self) -> std::time::Duration {
                (*self as u64).hours()
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
impl DurationExt for u64 {
    fn millis(&self) -> std::time::Duration {
        std::time::Duration::from_millis(*self)
    }
    fn seconds(&self) -> std::time::Duration {
        std::time::Duration::from_secs(*self)
    }
    fn minutes(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.seconds().as_secs() * 60)
    }
    fn hours(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.minutes().as_secs() * 60)
    }
}

impl_for!(u8, u16, u32, usize, i8, i16, i32, i64, isize, f64, f32);
