fn main() {
    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &s;
    // s3.push_str(" world");
    println!("{s2}");
}
