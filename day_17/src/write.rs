use std::fs::File;
use std::io::Write;

use array2d::Array2D;

use crate::component::Bloc;


pub fn write_output_file(output_path:String ,map: &Array2D<Bloc>, travel: &Vec<Bloc>) -> std::io::Result<()> {

    let mut output_file = File::create(output_path)?;

    for row in map.as_rows().iter() {

        for bloc in row.iter() {
            let score_to_write = get_charactere_to_write(&bloc, &travel);
            write!(output_file, "{}", score_to_write)?;
        }
        
       // let str: String = row.iter().map(|b| b.score.to_string()).collect();

        writeln!(output_file)?;
    }
    Ok(())
}

fn get_charactere_to_write(a: &Bloc, travel: &Vec<Bloc>) -> String {
    
    if travel.contains(a) {
        "#".to_string()
    } else {
        a.score.to_string()
    }
}