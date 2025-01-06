use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut alice = 0;
    let mut bob = 0;
    if a[0] < b[0] {
        bob += 1;
    } else if a[0] > b[0] {
        alice += 1;
    }
    
    if a[1] < b[1] {
        bob += 1;
    } else if a[1] > b[1] {
        alice += 1;
    }
    
    if a[2] < b[2] {
        bob += 1;
    } else if a[2] > b[2] {
        alice += 1;
    }
    
    let mut result = Vec::new();
    result.push(alice);
    result.push(bob);
    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
