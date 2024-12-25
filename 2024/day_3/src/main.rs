use component::Mul;
use transformer::transform_line;
mod reader;
mod component;
mod transformer;

fn main() {

    let input_path = String::from("./input/input.txt");

    let muls = transform_line(&input_path);

    let mut somme = 0 as i64;
    for mul in muls {
        somme = somme + mul.value;
    }

    println!("{}",somme);
}
