use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Counts work in progress that the app must finish before closing itself,
/// such as a content install that keeps running after the command that
/// started it has returned.
#[derive(Default)]
pub(crate) struct BackgroundWork(Arc<AtomicUsize>);

impl BackgroundWork {
    /// Marks work as in progress until the returned guard is dropped.
    pub(crate) fn start(&self) -> BackgroundWorkGuard {
        self.0.fetch_add(1, Ordering::SeqCst);
        BackgroundWorkGuard(self.0.clone())
    }

    #[cfg(feature = "tauri")]
    pub(crate) fn is_idle(&self) -> bool {
        self.0.load(Ordering::SeqCst) == 0
    }
}

pub(crate) struct BackgroundWorkGuard(Arc<AtomicUsize>);

impl Drop for BackgroundWorkGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}
