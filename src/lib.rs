use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // convert strings to lower case
    let input: Vec<String> = input.iter().map(|s| s.to_lowercase()).collect();

    // create an Atomic Reference Counting for concurrent situations
    let pointer = Arc::new(input);

    // create a vector of threads
    let mut threads = vec![];

    // split the task between every thread available (worker)
    for w in 0..worker_count {
        let pointer = Arc::clone(&pointer);
        let thread = thread::spawn(move || {
            // each thread will count the string at "job" index from the string vector 
            let mut job = w;
            let mut letters = HashMap::new();            
            while job < pointer.len() {
                // count logic, saving the result in a hashmap, ignoring non alphabetical characters
                for c in pointer
                    .get(job)
                    .unwrap()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                {
                    let counter = letters.entry(c).or_insert(0);
                    *counter += 1;
                }
                // update the job index
                job += worker_count;
            }
            letters
        });
        // join the results from each thread in a single vector
        threads.push(thread);
    }

    // bring together each counting in a single hashmap
    let mut solution = HashMap::new();
    for thread in threads {
        for (k, v) in thread.join().unwrap() {
            *solution.entry(k).or_insert(0) += v;
        }
    }
    solution
}
