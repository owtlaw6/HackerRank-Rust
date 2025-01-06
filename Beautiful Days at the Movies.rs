use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'beautifulDays' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER i
 *  2. INTEGER j
 *  3. INTEGER k
 */

fn beautifulDays(i: i32, j: i32, k: i32) -> i32 {
    let mut days = 0;
    for element in i..(j + 1) {
        let mut aux = element;
        let mut rev = 0;
        while aux > 0 {
            rev = rev * 10 + aux % 10;
            aux /= 10;
        }
        if (element - rev).abs() % k == 0 {
            days += 1;
        }
    }
    
    return days;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let i = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let j = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let result = beautifulDays(i, j, k);

    writeln!(&mut fptr, "{}", result).ok();
}
