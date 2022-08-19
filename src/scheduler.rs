use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::TimeUnits;
use log::info;

#[derive(Clone)]
pub(crate) struct Scheduler {
    scheduler: Arc<Mutex<clokwerk::Scheduler>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            scheduler: Arc::new(Mutex::new(clokwerk::Scheduler::new())),
        }
    }

    pub fn add_system_jobs(&mut self) {
        self.scheduler.lock().unwrap().every(5.minutes()).run(|| {
            println!("system job");
        });
        // Update checker
        self.scheduler.lock().unwrap().every(1.hour()).run(|| {
            println!("update checker");
        });
    }

    pub fn start(&mut self) {
        info!("Starting scheduler");
        let scheduler = self.scheduler.clone();
        // Start a thread and manually run the scheduler in an event loop
        std::thread::spawn(move || {
            // LOCK AND UNLOCK INSIDE THE LOOP, so we can use the scheduler while the thread is sleeping
            loop {
                {
                    let mut scheduler = scheduler.lock().unwrap();
                    scheduler.run_pending();
                }
                // Sleep 100ms
                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }
}