use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_vec = read_input();

    if input_vec.is_empty() {
        panic!("Wrong input");
    }

    let n = input_vec[0].len();

    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0..n {
        let mut ones = 0;
        let mut zeros = 0;
        for item in input_vec.iter() {
            if item.chars().nth(i).unwrap_or('0') == '1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        res1 *= 2;
        res2 *= 2;

        if ones > zeros {
            res1 += 1;
        } else {
            res2 += 1;
        }
    }

    println!("Result is {}", res1 * res2);
}

fn read_input() -> Vec<String> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./sampleinput.txt") {
        for line in lines {
            if let Ok(number) = line {
                res.push(number);
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
