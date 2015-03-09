// Multithreading with Rust
#![feature(std_misc)]
#![feature(old_io)]

use std::thread;

fn main() {
    // Initalizing two new threads threadA and threadB
    let threadA = thread::Builder::new().name("child1".to_string());
    let threadB = thread::Builder::new().name("child2".to_string());

    // Thread A is spwaned with a movable closure to print 1 to 10.
    // Moving closure is another type of closure which takes ownership of all variables it uses.
    guard.spawn(move || {
        for i in 1..10 {
            println!("Thread A {}", i)
        }
    });

    // Thread B spwaned with a movable closure to print 1 to 10.
    newThread.spawn(move || {
        for i in 1..10 {
            println!("Thread B {} ", i)
        }
    });
}
