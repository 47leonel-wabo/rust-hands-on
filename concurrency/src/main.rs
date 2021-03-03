use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    /*
        Thread - spawn
        the spawn function take a closure as parameter
        this closure represents the code that will be executed into the thread
     */
    spawn(|| {
        println!("This is from separate thread!");
        for n in 1..20 {
            println!("** child thread -> {}", &n);
        }
    }).join().unwrap(); // Join is used to make sure that parent will wait for its child to finish execution

    println!("This is from main thread!");
    for n in 1..5 {
        println!("main thread -> {}", &n);
        sleep(Duration::from_millis(1));  // we sleep the main thread for 1/3 of a second
    }

    /*
        Channel
        Transmitter (Upstream) =====>>====>>=====>>== Receiver (Downstream)
        mpsc => Multiple Producer, Single Consumer
        Massaging: one of the popular approach to ensure safe communication between threads
     */

    let (transmitter, receiver) = channel(); // channel creation

    spawn(move || {
        let value = String::from("hello");
        transmitter.send(value).unwrap(); // sending data from a thread (child thread)
    });

    // let received_data = receiver.try_recv().unwrap(); // Doesn't block and wait data
    let received_data = receiver.recv().unwrap(); // Block and wait data
    println!("Data received is: {}", received_data);
}
