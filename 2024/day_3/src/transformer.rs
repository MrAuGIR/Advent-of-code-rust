use crate::component::Mul;
use crate::reader::read_lines;
use std::io::{BufRead,Lines};
use regex::Regex;

pub fn transform_line(path: &String) -> Vec<Mul>  {
    let mut muls: Vec<Mul> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        
        create_mul(lines, &mut muls)
    }
    return muls;
}

fn create_mul<B: BufRead>(lines: Lines<B>, muls: &mut Vec<Mul>) {

    let pattern = r"mul\((\d+),(\d+)\)";

    let r = Regex::new(pattern).unwrap();

    for line in lines {

        if let Ok(line) = line {

            for caps in r.captures_iter(&line) {

                let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let mul = Mul { equation: " ".to_string(), value: a as i64 *b as i64 };
                muls.push(mul.clone());
            }
        }
    }
}