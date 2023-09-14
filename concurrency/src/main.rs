//use std::thread;
//use std::sync::mpsc;
//use std::time::Duration;
//use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use num::{BigUint, One};
use std::time::Instant;

fn factorial(num:u32) -> BigUint {
   if num == 0 || num == 1 {
       return BigUint::one();
   } else {
       (1..=num).map(BigUint::from).reduce(|a, b| a * b).unwrap()
   }
}

fn multi_fact (num: u32) -> BigUint {
    if num==0 || num ==1 {
        return BigUint::one();
    } else {
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |a, b| a * b)
    }
}

fn main() {

    let now = Instant::now();
    factorial(80000);
    println!("Time taken for single threaded factorial: {} seconds", now.elapsed().as_secs());


    let now = Instant::now();
    multi_fact(80000);
    println!("Time taken for multi threaded factorial: {} seconds", now.elapsed().as_secs());

    // std::thread::spawn(|| {
    //     println!("Hello from a thread!");
    // });

    // thread::sleep(Duration::from_millis(50));

    // println!("Hello from main!");

    // let handle = thread::spawn(|| {
    //     println!("Hello from a thread!");
    // });

    // handle.join().unwrap();

    // println!("Hello from main!");

    // let v = vec![1, 2, 3];

    // let handle = std::thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // let mut thread_handles = Vec::new();
    // for e in v {
    //     thread_handles.push(thread::spawn(move || {
    //         println!("Here's a vector: {:?}", e);
    //     }));
    // }

    // println!("Main thread");

    // for handle in thread_handles {
    //     handle.join().unwrap();
    // }


    // =============================================================
    // =============================================================


    // let (transmitter, receiver) = mpsc::channel();
    // let (transmitter, receiver) = mpsc::sync_channel(100);
    // let tx = transmitter.clone();


    // =============================================================
    // =============================================================


    // let val = String::from("Transmitting");
    // std::thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = receiver.recv().unwrap();
    // println!("Message: {}", msg);

    // =============================================================
    // =============================================================

    // std::thread::spawn(move || {
    //     let vec = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vec {
    //         transmitter.send(val).unwrap();
    //     }
    // });

    // std::thread::spawn(move || {
    //     let vec = vec![
    //         String::from("clone"),
    //         String::from("says"),
    //         String::from("hi"),
    //     ];

    //     for val in vec {
    //         tx.send(val).unwrap();
    //     }
    // });

    // for rec in receiver {
    //     println!("Message: {}", rec);
    // }


    // =============================================================
    // =============================================================

    // let rc1 = Arc::new(String::from("Hello"));
    // let rc2 = rc1.clone();

    // std::thread::spawn(move || {
    //     drop(rc2); 
    // });

    // =============================================================
    // =============================================================


    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);

    //     let handle = std::thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     }); // lock is given up here
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    // =============================================================
    // =============================================================

    // let lock = Arc::new(Mutex::new(0));
    // let lock2 = Arc::clone(&lock);
    
    // let _ = std::thread::spawn(move || {
    //     let _guard = lock2.lock().unwrap(); //we acquire the lock here
    //     panic!() //mutex is now poisoned
    // }).join();


    // // you should use match in the event you think of your threads might panic
    // let mut guard = match lock.lock(){
    //     Ok(guard) => guard, 
    //     Err(poisoned) => poisoned.into_inner(), //we recover from the poison here
    // };

    // *guard += 1;
    // println!("Result: {}", *guard);


}
