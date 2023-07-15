use std::thread;

use ansi_term::Color;
use tokio::{
    join, select,
    time::{sleep, Duration},
};

async fn task1() {
    println!("{}", Color::Cyan.paint("task1 start (0s)"));
    sleep(Duration::from_secs(1)).await;
    println!("{}", Color::Cyan.paint("task1 step1 (1s)"));
    thread::sleep(Duration::from_secs(2));
    println!("{}", Color::Cyan.paint("task1 end (3s)"));
}

async fn task2() {
    println!("{}", Color::Green.paint("task2 start"));
    sleep(Duration::from_secs_f32(0.5)).await;
    println!("{}", Color::Green.paint("task2 step1 (0.5s)"));
    sleep(Duration::from_secs(1)).await;
    println!("{}", Color::Green.paint("task2 end (1.5s)"));
}

async fn task3() {
    println!("{}", Color::Yellow.paint("task3 start"));
    thread::sleep(Duration::from_secs(1));
    println!("{}", Color::Yellow.paint("task3 end (1s)"));
}

#[tokio::main]
async fn main() {
    join!(task1(), task2(), task3());

    for _ in 0..3 {
        println!("-----------------------------------------------");

        select! {
            _ = task1() => println!("task1 done"),
            _ = task2() => println!("task2 done"),
        }
    }

    for _ in 0..3 {
        println!("-----------------------------------------------");

        select! {
            _ = task1() => println!("task1 done"),
            _ = task2() => println!("task2 done"),
            _ = task3() => println!("task3 done"),
        }
    }
}
