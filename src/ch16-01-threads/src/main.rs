use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // The spawned thread died when main exit
    // Use join to wait for handle thread
    // handle.join().unwrap();

    // Invalid
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    // drop(v); // Because this might happen
    // handle.join().unwrap();

    // Use move to move v to thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
