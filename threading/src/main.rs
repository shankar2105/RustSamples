#![feature(std_misc)]
#![feature(old_io)]

use std::thread;
use std::time::duration::Duration;
use std::old_io::timer;

fn main() {
    let guard = thread::Builder::new().name("child1".to_string());
    let newThread = thread::Builder::new().name("child2".to_string());

    guard.spawn(move || {
        for i in 1..10 {
            println!("Thread A {}", i)
        }
    });
    newThread.spawn(move || {
        for i in 1..10 {
            println!("Thread B {} ", i)
        }
    });
    timer::sleep(Duration::milliseconds(1000));
}
