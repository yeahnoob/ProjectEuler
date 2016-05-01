use std::env;

fn main() {
    let arg1 = env::args()
                   .nth(1)
                   .expect("not find the argument of max number in Problem_1");
    let mut sum = 0;
    for m in 1..arg1.parse::<u32>().expect("only integer number argument permited") {
        if m % 5 == 0 || m % 3 == 0 {
            sum += m;
        }
    }
    println!("The Sum of all the Multiple of 3 and 5 under {} is {}",
             arg1,
             sum);
    return;
}
