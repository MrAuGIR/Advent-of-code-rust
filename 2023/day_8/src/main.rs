mod reader;
mod components;
mod transformer;

use std::collections::HashMap;

use components::Itineraire;
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

    // les itineraires
    let mut itineraires: Vec<Itineraire> = Vec::new();

    // lecture des fichiers
    if let Ok(lines) = read_lines(path_instruction) {
        instructions = create_instructions(lines);

        println!("{:#?}", instructions);
    }

    if let Ok(lines) = read_lines(path_nodes) {
        graph = create_graph(lines, &mut itineraires);

       // println!("{:#?}", graph);
    }
    parcoure_du_graph(&graph, &mut itineraires, instructions)
    
}


pub fn parcoure_du_graph<'a>(graph:&'a HashMap<String, Node>, itineraires:&'a mut Vec<Itineraire<'a>>,instructions: String )
{

    for itineraire in itineraires.iter_mut() {
        itineraire.init_start(graph);
        println!("point de départ {:?}", itineraire.get_current_node());
    }

    let mut instruction_iterator = InstructionIterator::new(&instructions);

    let mut ending = false;

    let value_finish = itineraires.len() * 10;
    let mut counter = 0usize;

    while ending == false 
    {
        let direction = instruction_iterator.next().unwrap();
        counter = 0usize;

        let mut itineraires_iter = itineraires.iter_mut();
        let mut current_iters = itineraires_iter.next();

        println!("direction {:?}",direction);

        'iterA: while let Some(itineraire) = current_iters {
            itineraire.move_to_node(&direction);

            if itineraire.current_node.label.ends_with("Z") {
                counter += 10;
                println!("Il y a un z, counter value {:?}",counter);
                if value_finish == counter {
                    ending = true;
                    break 'iterA;
                }

                if counter > value_finish {
                    panic!("error value counter");
                }
            }

            current_iters = itineraires_iter.next();
        }
    }
    let counter_step = itineraires.get(0).unwrap().counter_step as u32;

    println!("etape {:?}",counter_step);

}
