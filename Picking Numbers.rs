use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pickingNumbers' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn pickingNumbers(a: &[i32]) -> i32 {
    let mut maxLen = 0;

    for &num in a {
        let mut count0 = 0;
        let mut count1 = 0;

        for &x in a {
            if x == num {
                count0 += 1;
            } else if x == num + 1 {
                count1 += 1;
            }
        }

        let total = count0 + count1;
        if total > maxLen {
            maxLen = total;
        }
    }

    return maxLen
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pickingNumbers(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
