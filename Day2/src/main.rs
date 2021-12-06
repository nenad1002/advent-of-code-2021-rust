use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_vec = read_input();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for item in input_vec {
        match item.0.as_str() {
            "forward" => {
                x += item.1;
                y += aim * item.1;
            }
            "down" => {
                aim += item.1;
            }
            "up" => {
                aim -= item.1;
            }
            _ => {}
        }
    }

    println!("Result is {}", x * y);
}

fn read_input() -> Vec<(String, i32)> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let elements: Vec<_> = line.split(" ").into_iter().collect();
                if elements.len() > 2 {
                    panic!("There are more than 2 elements");
                }
                res.push((elements[0].to_owned(), elements[1].parse::<i32>().unwrap()));
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
