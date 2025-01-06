use std::io::{self, BufRead};
extern crate num;

//use num::BigUint;
use num::{BigUint, FromPrimitive};

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut sol = BigUint::from(1u32);

    for number in 1..=n {
        sol = sol * BigUint::from_i32(number).unwrap();
    }
    println!("{}", sol);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
