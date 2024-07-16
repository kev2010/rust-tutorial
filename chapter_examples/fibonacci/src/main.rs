use std::io;

fn main() {
    println!("Which Fibonacci number do you want to see? To see the first, type '1', not '0'.");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: usize = n.trim().parse().expect("Please type a number!");

    if n == 0 {
        println!("Please type a number greater than 0!");
        return;
    }
    println!("The {n}th Fibonacci number is {}", get_fibonacci_number(n));
}

fn get_fibonacci_number(n: usize) -> usize {
    if n == 1 || n == 2 {
        return 1;
    }
    get_fibonacci_number(n - 1) + get_fibonacci_number(n - 2)
}
