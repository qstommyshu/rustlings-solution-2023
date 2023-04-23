// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value

            // Explanation: Need to lock the Mutex before updating shared_variable
            //So that other threads cannot mutate the value
            // This is the syntax I found in the rust official documentation

            // lock() returns a Result type, we need to use unwrap() to get the
            //value of the result.The value of the Mutex is JobStatus, then we
            //use the JobStatus struct to access its field.
            let mut status_child = status_shared.lock().unwrap();
            status_child.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything

        // Don't really understand why do we need to unwrap status.lock() here, need to
        //circle back

        // Circled back: handle.join().unwrap() means all the children processes are finished.
        //However, in the initialization of variable status, it is Arc(Mutex(JobStatus)),
        //we need to use status.lock().unwrap() to take JobStatus out from Mutex, then
        //access the filed inside.
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
        // interesting in the output? Do you have to 'join' on all the handles?
    }
}
