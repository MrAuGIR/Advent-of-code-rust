use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


fn main() {
    
    let path = String::from("./input/calibration.txt");

    let nombres_alphanumeriques = ["one","two","three","four","five","six", "seven", "eight", "nine"];

 //   let regex = Regex::new(&format!(r"\d|(?:{})", nombres_alphanumeriques.join("|"))).expect("erreur dans la regex");
    let regex = Regex::new(&format!(r"(?:{})|\d", nombres_alphanumeriques.join("|"))).expect("erreur dans la regex");

    let mut count: i32 = 0;

    if let Ok(lines) = read_lines(path.to_owned()) {
        for line in lines {
            if let Ok(ip) = line   {
                println!("{}",ip);

                
                let nombres: Vec<&str> = regex.find_iter(&ip).map(|m| m.as_str()).collect();
                
                println!("premier {:?}, dernier {:?}",nombres.first(), nombres.last());

                let value: i32 = match nombres.len() {
                    n if n >= 1 => {
                        let chiffre = format!("{}{}",convert_alpha(nombres[0]), convert_alpha(nombres[nombres.len() - 1]));
                        chiffre.parse::<i32>().expect("expect a number")
                    }
                    _ => {
                      //  println!("rien trouve sur cette ligne : {}",ip);
                        0
                    }
                };
               // println!("valeur de la ligne est {:?}",value);
                count += value;
            
            }  
        }
        println!("{:?}",count);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_alpha(alpha_number: &str) -> &str {
    return match alpha_number  {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => alpha_number
    }
}

fn find_first_last_occurence<'a>(line: &'a str, numbers: &[&'a str]) -> (Option<&'a str>, Option<&'a str>) {

    let mut first: Option<&str> = None;
    let mut last: Option<&str> = None;

    for &number in numbers {
        if let Some(index) = line.find(number) {
            if first.is_none() || index < line.find(first.unwrap()).unwrap() {
                first = Some(number);
            }
            let end_index = index + number.len();
            if last.is_none() || end_index > line.find(last.unwrap()).unwrap() {
                last = Some(number);
            }
        }
        
    }

    (first,last)
}