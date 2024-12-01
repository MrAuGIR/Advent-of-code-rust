use std::collections::HashMap;


#[derive(Debug,Clone,Default)]
pub struct Node {
    pub label: String,
    pub left: String,
    pub right: String
}

impl Node {

    pub fn new(label:String, left: String, right: String) -> Node {
        Node {
            label: label,
            left: left,
            right: right
        }
    }
}

pub struct InstructionIterator<'a> {
    instructions: &'a str,
    pub index: usize,
}

impl<'a> InstructionIterator<'a> {

    pub fn new(instructions: &'a str) -> Self {
        InstructionIterator{instructions, index: 0}
    }
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.instructions.is_empty() {
            return  None;
        }

        let currrent_char = self.instructions.chars().nth(self.index)?;
        self.index = (self.index + 1) % self.instructions.len();
        Some(currrent_char)
    }
}

#[derive(Debug)]
pub struct Itineraire<'a>{
    pub start_node: String,
    pub end_node: String,
    pub graph: Option<&'a HashMap<String, Node>>,
    pub current_node: Node,
    pub counter_step: usize,
}

impl<'a> Itineraire<'a> {

    pub fn new(start_node: String, end_node: String, current_node: Node) -> Self {
        Itineraire{start_node, end_node, graph:None,current_node,counter_step: 0}
    }

    pub fn init_start(&mut self,graph: &'a HashMap<String, Node>) {
        self.graph = Some(graph);
        self.current_node = self.graph.as_ref().unwrap().get(self.start_node.as_str()).unwrap().clone();
        self.counter_step = 0;
    }

    pub fn get_current_node(&self) -> &Node {
        return self.graph.as_ref().unwrap().get(self.current_node.label.as_str()).unwrap();
    }

    pub fn get_node_by_label(&self, label: String) -> &Node {
        return self.graph.as_ref().unwrap().get(label.as_str()).unwrap();
    }

    pub fn move_to_node(&mut self, direction: &char)
    {
        let key_node = match direction {
            'L' => {
                self.current_node.left.clone()
            },
            'R' => {
                self.current_node.right.clone()
            },
            _ => panic!("Instruction incorrecte")
        };
        
        self.current_node = self.get_node_by_label(key_node).clone();

        self.counter_step += 1;
        println!("step : {:?}",self.counter_step);
    }
}