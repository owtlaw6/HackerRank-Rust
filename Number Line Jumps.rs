use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'kangaroo' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER x1
 *  2. INTEGER v1
 *  3. INTEGER x2
 *  4. INTEGER v2
 */

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let mut posK1 = x1;
    let mut posK2 = x2;
    let mut distance = if x1 > x2 {x1 - x2} else {x2 - x1};

    while posK1 != posK2 {
        posK1 += v1;
        posK2 += v2;
        
        if posK1 > posK2 {
            if (posK1 - posK2) < distance {
                distance = posK1 - posK2;
            } else {
                return "NO".to_string();
            }
        } else if (posK2 - posK1) < distance {
            distance = posK2 - posK1;
        } else {
            return "NO".to_string();
        }
    }
    return "YES".to_string();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}
