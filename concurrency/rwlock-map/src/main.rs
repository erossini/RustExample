use std::collections::HashMap; use std::sync::{Arc, RwLock}; use std::{thread, time::Duration};
fn main() {
    let map = Arc::new(RwLock::new(HashMap::<String,i32>::new()));
    map.write().unwrap().insert("x".into(), 0);
    let m1 = Arc::clone(&map);
    let writer = thread::spawn(move || {
        for i in 0..5 {
            thread::sleep(Duration::from_millis(100));
            *m1.write().unwrap().get_mut("x").unwrap() = i;
        }
    });
    let m2 = Arc::clone(&map);
    let reader = thread::spawn(move || {
        for _ in 0..10 { println!("x = {}", *m2.read().unwrap().get("x").unwrap()); thread::sleep(Duration::from_millis(50)); }
    });
    writer.join().unwrap(); reader.join().unwrap();
}
