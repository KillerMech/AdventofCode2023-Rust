use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_1(){
    let mut numbers: Vec<i32> = Vec::new();
    let mut first_last: Vec<char> = Vec::new();
    let mut total: i32 = 0;
    let mut num_str: String = String::new();
    
    if let Ok(lines) = read_lines("day_1_input.txt") {
        for line in lines {
            for ch in line.unwrap().chars() {
                if ch.is_digit(10) {
                    if first_last.len() == 0 {
                        first_last.push(ch);
                        first_last.push(ch);
                    }
                    else {
                        first_last[1] = ch;
                    }
                }
            }
            for ch in first_last.iter() {
                num_str.push(*ch);
            } 
            first_last.clear();
            numbers.push(num_str.parse::<i32>().unwrap());
            num_str.clear();
        }
    }
    for num in numbers.iter() {
        total += num;
    }
    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
