pub trait Duration {
    fn millis(&self) -> std::time::Duration;
    fn seconds(&self) -> std::time::Duration;
    fn minutes(&self) -> std::time::Duration;
    fn hours(&self) -> std::time::Duration;
}
impl Duration for u64 {
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
