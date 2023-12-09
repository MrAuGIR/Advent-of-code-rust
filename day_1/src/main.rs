use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;



fn main() {
    
    let path = String::from("./input/data.txt");

    let nombres_alphanumeriques = ["1","2","3","4","5","6","7","8","9","one","two","three","four","five","six", "seven", "eight", "nine"];

    //let regex = Regex::new(&format!(r"\d|(?:{})", nombres_alphanumeriques.join("|"))).expect("erreur dans la regex"); // regex methode premiere partie

    let mut count: i32 = 0;

    if let Ok(lines) = read_lines(path.to_owned()) {
        for line in lines {
            if let Ok(ip) = line   {
                println!("{}",ip);

               // let nombres: Vec<&str> = regex.find_iter(&ip).map(|m| m.as_str()).collect(); // utilisé lors de la première partie

                let (first, last) = find_first_last_occurence(&ip, &nombres_alphanumeriques);
                
                println!("premier {:?}, dernier {:?}",first, last);


                let value: i32 = if let Some(first) = first {
                    if let Some(last) = last {
                        let chiffre = format!("{}{}",convert_alpha(first), convert_alpha(last));
                        chiffre.parse::<i32>().unwrap_or(0)
                    } else {
                        convert_alpha(first).parse::<i32>().unwrap_or(0)
                    }
                } else {
                    0
                };

                println!("valeur de la ligne est {:?}",value);
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

    
    let mut indices_map: HashMap<usize, &'a str> = HashMap::new();

    for &number in numbers {

        indices_map.extend(line.match_indices(number).map(|(i,n)| (i,n)));
    }
    
    let mut indices: Vec<usize> = indices_map.keys().cloned().collect();
    indices.sort();

    if let Some(first_index) = indices.first() {
        first = indices_map.get(first_index).cloned();
    }

    if let Some(last_index) = indices.last() {
        last = indices_map.get(last_index).cloned();
    }

    println!("{:?}", indices_map);

    (first, last)
}