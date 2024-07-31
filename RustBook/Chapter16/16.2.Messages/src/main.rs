use macros::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n=========Running {}", function!());
    listing_16_7();
    listing_16_10();
    q1();
    q2();
}

fn listing_16_7() {
    println!("\n=========Running {}", function!());
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn listing_16_10() {
    println!("\n=========Running {}", function!());
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}

enum ClientMessage { Incr, Get, Quit }
enum ServerMessage { Get(usize) }

fn q1() {
    println!("\n=========Running {}", function!());
    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
            }
        }
    });
    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }
    #[allow(irrefutable_let_patterns)]
    if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
        println!("{}", n)
    }
    server.join().unwrap();
}

fn q2() {
    println!("\n=========Running {}", function!());
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("Hello world");
        tx.send(s.clone()).unwrap();
        tx.send(s.len().to_string()).unwrap();
    });
    let s = rx.recv().unwrap();
    let n = rx.recv().unwrap();
    println!("{s} {n}");
}
