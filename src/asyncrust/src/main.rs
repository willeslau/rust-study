#![feature(sync_unsafe_cell)]

mod executor;
mod timer;

use crate::timer::Sleep;
use executor::new_executor_and_spawner;
use std::time::Duration;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // More on how `async` gets processed by compiler: https://www.youtube.com/watch?v=ZHP9sUqB3Qs
    spawner.spawn(async move {
        println!("starting");
        Sleep::new(Duration::new(1, 0)).await;
        println!("wake up!")
    });

    executor.run();
}
