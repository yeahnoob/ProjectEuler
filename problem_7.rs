fn main() {
    let count_max = 10001;
    //let count_max = 1000001;
    let mut count: i64 = 0;
    let mut lastn: u64 = 1;
    while count < count_max {
        lastn += 2;
        count += is_prime(lastn);
    }
    println!("The {} count prime number is {}", count_max, lastn);
}

fn is_prime(x: u64) -> i64 {
    let mut i: u64 = 2;
    while i * i <= x {
        if x % i == 0 {
            return 0;
        }
        i += 1;
    }

    return 1;
}
