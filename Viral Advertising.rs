use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'viralAdvertising' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn viralAdvertising(n: i32) -> i32 {
    let mut shared = 5;
    let mut liked = 2;
    let mut cumuluative = 2;
    
    for element in 1..n {
        shared = shared / 2;
        shared = shared * 3;
        liked = shared / 2;
        cumuluative += liked;
    }
    
    return cumuluative;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = viralAdvertising(n);

    writeln!(&mut fptr, "{}", result).ok();
}
