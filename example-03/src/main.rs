fn main() {
    let arr_1 = [1,2,3,4];
    println!("1st element : {}", arr_1[0]);
    println!("Length : {} ", arr_1.len());

    let mut s1 = String::from("Hello");
    change_string(&mut s1);
    println!("s1 = {}", s1);
}

fn change_string(s: &mut String) {
    s.push_str(", Rust!");
}

// iB u8 (signed unsigned 8-bit integer)
// i16 u16 (signed unsigned 16-bit integer)
// i32 u32 (signed unsigned 32-bit integer)
// i64 u64 (signed unsigned 64-bit integer)
// i128 u128 (signed unsigned 128-bit integer)
// isize usize (signed unsigned size)