use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> String
where P: AsRef<Path>, {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    
    if let Err(error) = file.read_to_string(&mut buffer) {
        panic!("{:?}",error);
    }

    buffer
}