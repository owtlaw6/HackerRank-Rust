use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut total: i64 = 0;
    
    for element in arr {
        total += *element as i64;
    }

    let mut min = i64::MAX;
    let mut max = i64::MIN;
    
    for number in arr {
        let current_sum = total - *number as i64;
        if current_sum < min {
            min = current_sum;
        }
        if current_sum > max {
            max = current_sum;
        }
    }
    
    println!("{min} {max}");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
