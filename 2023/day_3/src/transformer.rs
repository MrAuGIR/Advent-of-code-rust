use crate::components::*;
use std::fs::File;
use std::io::{self, Lines};


pub fn transform_to_graph(lines: Lines<io::BufReader<File>>) -> Graph {
    let mut graph = Graph::new();
    let mut max_index_line = 0u16;
    for (line_index,line) in lines.enumerate() {
        max_index_line = line_index as u16;
        // println!("{:#?}", line);
        if let Ok(line_content) = line {
            let mut current_part_number = String::new();

            graph.set_max_lenth(line_content.len() as u16);

            for (char_index,character) in line_content.chars().enumerate() {
                
                match character {
                    '.' => {
                        if !current_part_number.is_empty() {
                            let type_node = Type::PartNumber(current_part_number.parse::<u32>().unwrap());
                            graph.add_node(create_node(type_node, char_index - current_part_number.len(), line_index));
                        };
                        current_part_number.clear();
                        graph.add_node(create_node(Type::Other(character.to_string()), char_index, line_index));
                    },
                    '*' => {
                        if !current_part_number.is_empty() {
                            let type_node = Type::PartNumber(current_part_number.parse::<u32>().unwrap());
                            graph.add_node(create_node(type_node, char_index - current_part_number.len(), line_index));
                        };
                        current_part_number.clear();
                        graph.add_node(create_node(Type::Gear(character.to_string()), char_index, line_index));
                    },
                    _ if character.is_numeric() => {
                        current_part_number.push(character);
                        continue; 
                    },
                    _ if character.is_ascii_punctuation() => {
                        if !current_part_number.is_empty() {
                            let type_node = Type::PartNumber(current_part_number.parse::<u32>().unwrap());
                            graph.add_node(create_node(type_node, char_index - current_part_number.len(), line_index));
                        };
                        current_part_number.clear();
                        graph.add_node(create_node(Type::Symbol(character.to_string()), char_index, line_index));
                    },
                    _ => {
                        if !current_part_number.is_empty() {
                            let type_node = Type::PartNumber(current_part_number.parse::<u32>().unwrap());
                            graph.add_node(create_node(type_node, char_index - current_part_number.len(), line_index));
                        };
                        current_part_number.clear();
                        graph.add_node(create_node(Type::Symbol(character.to_string()), char_index, line_index));
                    },
                };
            }
            if !current_part_number.is_empty() {
                let type_node = Type::PartNumber(current_part_number.parse::<u32>().unwrap());
                graph.add_node(create_node(type_node, graph.max_lenth as usize - current_part_number.len(), line_index));
            }
        }
    }
    graph.set_max_index_line(max_index_line);
    return graph;
}

pub fn create_node(node_type: Type,char_index: usize, line_index: usize) -> Node {
    Node::new(node_type, Point(char_index as u16,line_index as u16))
}
