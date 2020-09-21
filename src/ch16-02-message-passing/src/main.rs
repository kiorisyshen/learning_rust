use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = String::from("hi_1");

        println!("Start thread1");
        thread::sleep(Duration::from_secs(1));
        tx.send(val).unwrap();
        println!("End val in thread1");
        // println!("val is {}", val); // cant use after sent

        let vals = vec![
            String::from("hi_2"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("End thread1");
    });

    thread::spawn(move || {
        println!("Start thread2");
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("End thread2");
    });

    let received = rx.recv().unwrap();
    println!("Got_1: {}", received);

    let received = rx.recv().unwrap();
    println!("Got_2: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}
