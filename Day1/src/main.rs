use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_vec = read_input();
    let n = input_vec.len();

    if n < 3 {
        panic!("There is not enough elemets");
    }

    let mut res = 0;
    let mut curr_sum: i32 = input_vec[0..3].iter().sum();

    for i in 3..n {
        let prev_sum = curr_sum;
        curr_sum -= input_vec[i - 3];
        curr_sum += input_vec[i];
        if curr_sum > prev_sum {
            res += 1;
        }
    }

    println!("Result is {}", res);
}

fn read_input() -> Vec<i32> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(number) = line {
                res.push(number.parse::<i32>().unwrap());
            }
        }
    }

    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
