pub trait StdMutexExt<T> {
    fn lock_ignore(&self) -> std::sync::MutexGuard<T>;
}
pub trait StdRwLockExt<T> {
    fn read_ignore(&self) -> std::sync::RwLockReadGuard<T>;
    fn write_ignore(&self) -> std::sync::RwLockWriteGuard<T>;
}

impl<T> StdMutexExt<T> for std::sync::Mutex<T> {
    fn lock_ignore(&self) -> std::sync::MutexGuard<T> {
        self.lock().unwrap_or_else(|e| e.into_inner())
    }
}
impl<T> StdRwLockExt<T> for std::sync::RwLock<T> {
    fn read_ignore(&self) -> std::sync::RwLockReadGuard<T> {
        self.read().unwrap_or_else(|e| e.into_inner())
    }
    fn write_ignore(&self) -> std::sync::RwLockWriteGuard<T> {
        self.write().unwrap_or_else(|e| e.into_inner())
    }
}
