use reader::read_lines;
use transformer::get_map;


mod reader;
mod transformer;

fn main() {
    let input_path = "./input/calibration.txt";

    let content = read_lines(input_path);

    let map  = get_map(content.as_str());

    println!("{:?}",map);
}
