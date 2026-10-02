use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let nums: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![0]));
    let mut handles = Vec::new();

    for i in 1..=5 {
        let nums_cpy = Arc::clone(&nums);
        handles.push(thread::spawn(move || {
            nums_cpy.lock().unwrap().push(i);
        }));
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("{:?}", nums.lock().unwrap());
    let mut sorted = nums.lock().unwrap().clone();
    sorted.sort();
    println!("Sorted: {:?}", sorted);
}
