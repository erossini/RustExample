use std::thread;
fn main() {
    let v = vec![1,2,3,4,5];
    let handle = thread::spawn(move || v.iter().sum::<i32>());
    let sum = handle.join().unwrap();
    println!("sum = {}", sum);
}
