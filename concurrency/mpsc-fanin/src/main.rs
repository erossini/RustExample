use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let t = tx.clone();
        thread::spawn(move || t.send(format!("hello from {}", i)).unwrap());
    }
    drop(tx);
    for msg in rx {
        println!("{}", msg);
    }
}
