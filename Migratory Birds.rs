use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut bird1 = 0;
    let mut bird2 = 0;
    let mut bird3 = 0;
    let mut bird4 = 0;
    let mut bird5 = 0;
    
    for &element in arr {
        match element {
            1 => bird1 += 1,
            2 => bird2 += 1,
            3 => bird3 += 1,
            4 => bird4 += 1,
            5 => bird5 += 1,
            _ => print!(""),
        }
    }
    
    if bird1 >= bird2 && bird1 >= bird3 && bird1 >= bird4 && 
        bird1 >= bird5 {
        return 1;
    }
    if bird2 >= bird1 && bird2 >= bird3 && bird2 >= bird4 && 
        bird2 >= bird5 {
        return 2;
    }
    if bird3 >= bird1 && bird3 >= bird2 && bird3 >= bird4 && 
        bird3 >= bird5 {
        return 3;
    }
    if bird4 >= bird1 && bird4 >= bird2 && bird4 >= bird3 && 
        bird4 >= bird5 {
        return 4;
    }
    
    return 5;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}