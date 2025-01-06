use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for number in (1..n + 1) {
        for spaces in (1..(n - number + 1)){
            print!(" ");
        }
        for hashes in (1..number + 1){
            print!("#");
        }
        println!("");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
