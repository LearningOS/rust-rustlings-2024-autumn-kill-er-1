// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];

    // Spawning 10 threads
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()  // Returning the elapsed time in milliseconds
        }));
    }

    let mut results: Vec<u128> = vec![];

    // Collect the results from each thread
    for handle in handles {
        // Using handle.join() to wait for the thread to finish and collect its result
        results.push(handle.join().expect("Thread panicked!"));
    }

    // Ensure all 10 threads completed
    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    // Print the result for each thread
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
