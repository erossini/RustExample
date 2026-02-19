fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // "Ownership" of the "String" changes from s1 to s2
    println!("s1 = {}, s2 = {}", s1, s2);

    let age = 22;
    match age {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("OH!"),
        65..=i32::MAX => println!("Well done!"),
        _ => println!("Happy Birthday!!!"),
    };
}
