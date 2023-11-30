use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Check {
    keep_running: AtomicBool,
    max_errors_per_minute: u32,
    count: AtomicU32,
}

impl Check {
    pub fn new(max_errors_per_minute: u32) -> Arc<Self> {
        let check = Arc::new(Self {
            max_errors_per_minute,
            keep_running: AtomicBool::new(true),
            count: AtomicU32::new(0),
        });

        let c = check.clone();
        thread::spawn(move || {
            let check = Arc::clone(&c);

            while check.keep_running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(60));
                check.count.store(0, Ordering::Relaxed);
            }
        });

        check
    }

    pub fn error(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn is_limit(&self) -> bool {
        self.count.load(Ordering::Relaxed) < self.max_errors_per_minute
    }
}

impl Drop for Check {
    fn drop(&mut self) {
        self.keep_running.store(false, Ordering::Relaxed);
    }
}
