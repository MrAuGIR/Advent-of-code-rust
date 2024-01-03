use std::fs::File;
use std::io::{self, Lines, Write};


use crate::component::*;
use crate::reader::read_lines;


pub fn galaxies_map_transformer(lines: Lines<io::BufReader<File>>,galaxies: &mut Vec<Galaxy>){

    for (row, line) in lines.enumerate() {

        if let Ok(line_content) = line {

            

            for (col, caractere) in  line_content.chars().enumerate() {

                match caractere {
                    '#' =>  {
                        let galaxy = Galaxy::new(row, col);
                        galaxies.push(galaxy);
                    },
                    _ => {

                    }
                }
            }
        }
    }
}

pub fn expand_universe_raw(input: &str, output: &str) -> io::Result<()> {

    let lines = read_lines(input)?;

    let mut points: Vec<Point> = Vec::new();
    for (row, line) in lines.enumerate() {
        let line = line?;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.push(Point { row, col });
            }
        }
    }

    // Trouver les limites de l'univers
    let min_row = points.iter().map(|p| p.row).min().unwrap_or(0);
    let max_row = points.iter().map(|p| p.row).max().unwrap_or(0);
    let min_col = points.iter().map(|p| p.col).min().unwrap_or(0);
    let max_col = points.iter().map(|p| p.col).max().unwrap_or(0);

    // Ouvrir le fichier de sortie en écriture
    let mut output_file = File::create(output)?;

    // Écrire les lignes étendues dans le fichier de sortie
    for row in min_row..=max_row {

        let mut row_contains_galaxy = false;

        for col in min_col..=max_col {
            if points.iter().any(|p| p.row == row && p.col == col) {
                write!(output_file, "#")?;
                row_contains_galaxy = true;
            } else {
                write!(output_file, ".")?;
            }
        }

         // Dupliquer la ligne si elle ne contient pas de galaxie
        if !row_contains_galaxy {
            writeln!(output_file)?;

            for _ in min_col..=max_col {
                write!(output_file, ".")?;
            }
        }

        writeln!(output_file)?;
    }

    Ok(())

}

pub fn expand_universe_col(input: &str, output: &str) -> io::Result<()> {

    let lines = read_lines(input)?;

    let mut points: Vec<Point> = Vec::new();
    for (row, line) in lines.enumerate() {
        let line = line?;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.push(Point { row, col });
            }
        }
    }

    // Trouver les limites de l'univers
    let min_row = points.iter().map(|p| p.row).min().unwrap_or(0);
    let max_row = points.iter().map(|p| p.row).max().unwrap_or(0);
    let min_col = points.iter().map(|p| p.col).min().unwrap_or(0);
    let max_col = points.iter().map(|p| p.col).max().unwrap_or(0);

    let mut output_file = File::create(output)?;

    for col in min_col..=max_col {

        let mut col_contains_galaxy = false;

        for row in min_row..=max_row {
            if points.iter().any(|p| p.row == row && p.col == col) {
                write!(output_file, "#")?;
                col_contains_galaxy = true;
            } else {
                write!(output_file, ".")?;
            }
        }

         // Dupliquer la colonne si elle ne contient pas de galaxie
        if !col_contains_galaxy {
            writeln!(output_file)?;

            for _ in min_row..=max_row {
                write!(output_file, ".")?;
            }
        }

        writeln!(output_file)?;
    }

    Ok(())

}