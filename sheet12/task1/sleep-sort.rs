use std::thread;
use std::sync::{RwLock, Arc};
use std::time::Duration;

fn main() {
    let mut arr = [83, 12, 13, 35, 91, 71, 75, 58, 26, 38, 2, 23, 10];
    sleep_sort(&mut arr);
    assert_eq!(arr, [2, 10, 12, 13, 23, 26, 35, 38, 58, 71, 75, 83, 91]);
}

fn sleep_sort(arr: &mut [u64]) {
    let mut multiplier: u32 = 1;

    // Repeat this loop until the input array is sorted
    for count in 1.. {
        let timer = Arc::new(multiplier);
        let result = Vec::new();
        let my_result = Arc::new(RwLock::new(result));
        let mut children = vec![];

        // For every value in the input array, create a thread
        for index in 0..arr.len() {
            let mul = timer.clone();
            let dest = my_result.clone();
            let source = Arc::new(arr[index]);

            children.push(thread::spawn(move || {
                if let Ok(elem) = Arc::try_unwrap(source) {
                    // This is not so safe because we cast from u64 to u32, but it doesn't matter
                    // much in this example
                    thread::sleep(Duration::new(0, mul.saturating_mul(elem as u32)));
                    if let Ok(mut res) = dest.write() {
                        res.push(elem);
                    }
                }
            }));
        }
        // Wait for all threads to finish
        for child in children {
            let _ = child.join();
        }
        // Copy the result array to the input array
        if let Ok(lala) = Arc::try_unwrap(my_result) {
            if let Ok(lulu) = lala.read() {
                arr.copy_from_slice(&*lulu);
            }
        }
        // If the array is sorted, stop
        if sorted(arr) {
            println!("Needed {} loops to sort.", count);
            break;
        }
        // Mulitply thread sleep-multiplier by 2
        multiplier = multiplier.saturating_mul(2);
    }
}

/// Check whether array 'arr' is sorted
fn sorted(arr: &[u64]) -> bool {
    (0..arr.len()).all(|i| {
        if i < arr.len() - 1{
            arr[i] <= arr[i+1]
        } else {
            arr[i] >= arr[i-1]
        }})
}

