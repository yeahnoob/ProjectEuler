use std::env;

fn main() {
    let arg1 = env::args()
                   .nth(1)
                   .expect("not find the argument of max number in Problem_2");
    // println!("The max integer value(u64 Type) in Rust language is {}",
    //         u64::max_value());
    let mut sum: u64 = 0;
    let mut fib1: u64 = 1;
    let mut fib2: u64 = 2;
    let max = arg1.parse::<u64>().expect("only integer number argument permited");
    while fib2 < max {
        // println!("{} {}", fib1, fib2);
        if fib1 % 2 == 0 {
            sum += fib1;
        }
        if fib2 % 2 == 0 {
            sum += fib2;
        }
        fib1 = fib1 + fib2;
        fib2 = fib2 + fib1;
    }
    if fib1 < max && fib1 % 2 == 0 {
        // println!("{} {}, only the first value below {}", fib1, fib2, max);
        sum += fib1;
    }
    println!("The sum of even-valued term in the Fibonacci sequence(below {}) is {}",
             max,
             sum);

}
