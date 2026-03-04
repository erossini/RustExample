use std::{sync::Arc, thread};
fn main() {
    // Uncomment to see compile error: Rc is not Send/Sync
    // let r = Rc::new(String::from("hi"));
    // thread::spawn(move || println!("{}", r));

    let a = Arc::new(String::from("hi"));
    let a2 = Arc::clone(&a);
    thread::spawn(move || println!("{}", a2)).join().unwrap();
}
