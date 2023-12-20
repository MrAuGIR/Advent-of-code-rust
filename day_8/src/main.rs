mod reader;
mod components;
mod transformer;

use std::collections::HashMap;

use components::Node;

use crate::components::InstructionIterator;
use crate::reader::*;
use crate::transformer::*;

fn main() {
    // chemin des fichiers
    let path_instruction = String::from("./input/data/instructions.txt");
    let path_nodes = String::from("./input/data/nodes.txt");

    // les variables
    let mut instructions = String::new();
    let mut graph :HashMap<String, Node> = HashMap::new();

    // lecture des fichiers
    if let Ok(lines) = read_lines(path_instruction) {
        instructions = create_instructions(lines);

        println!("{:#?}", instructions);
    }

    if let Ok(lines) = read_lines(path_nodes) {
        graph = create_graph(lines);

        println!("{:#?}", graph);
    }


    parcoure_du_graph(graph, String::from("AAA"), instructions)
    
}


pub fn parcoure_du_graph(graph:HashMap<String, Node>, start_node: String,instructions: String )
{

    let mut key_node = start_node;
    println!("point de départ {:?}", key_node);

    let mut instruction_iterator = InstructionIterator::new(&instructions);

    let mut ending = false;
    let mut counter_step = 0u32;

    while ending == false 
    {
        let direction = instruction_iterator.next().unwrap();
        let current_node = graph.get(key_node.as_str()).unwrap();
        println!("instruction en cour {:?}", direction);

        key_node = match direction {
            'L' => {
                current_node.left.clone()
            },
            'R' => {
                current_node.right.clone()
            },
            _ => panic!("Instruction incorrecte")
        };
        println!("nouveau point {:?}", key_node);
        counter_step += 1;
        if key_node == "ZZZ" {
            ending = true;
        }
    }

    println!("etape {:?}",counter_step);

}