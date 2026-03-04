use std::thread;
fn main() {
    let h1 = thread::spawn(|| println!("Hello from thread A"));
    let h2 = thread::spawn(|| println!("Hello from thread B"));
    h1.join().unwrap(); h2.join().unwrap();
    println!("Back in main");
}
