// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

<<<<<<< HEAD
// I AM NOT DONE

use std::sync::Arc;
=======

use std::sync::{Arc, Mutex};
>>>>>>> 90fab5c (2022/7/17/14.12)
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
<<<<<<< HEAD
    let status = Arc::new(JobStatus { jobs_completed: 0 });
=======
    let status = Arc::new(Mutex::new(JobStatus {  jobs_completed: 0 }));
>>>>>>> 90fab5c (2022/7/17/14.12)
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
<<<<<<< HEAD
            status_shared.jobs_completed += 1;
        }
    });
    while status.jobs_completed < 10 {
=======
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
>>>>>>> 90fab5c (2022/7/17/14.12)
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
