

#[derive(Debug,PartialEq)]
pub enum Type {
    Symbol(String),
    PartNumber(u32),
    Other(String)
}

impl Type {
    pub fn get_integer_value(&self) -> Option<u32> {
        match self {
            Type::PartNumber(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_string_value(&self) -> Option<&String> {
        match self {
            Type::Symbol(value) => Some(value),
            Type::Other(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Point(pub u16,pub u16);

#[derive(Debug)]
pub struct Node {
    pub position: Point,
    pub node_type: Type
}

impl Node {
    pub fn new(type_node: Type, position: Point) -> Node {
        Node {
            position: position,
            node_type: type_node,
        }
    }

    pub fn is_real_part_number(&self, graph: &Graph) -> bool {

        return self.node_symbole_in_line(graph);
    }

    fn node_symbole_in_line(&self, graph: &Graph) -> bool {

        let lenth = self.node_type.get_integer_value().unwrap().to_string().len() - 1;

        // point de départ axe x
        let mut x_start = self.position.0;
        if self.position.0 > 0 {
            x_start -= 1u16;
        }

        // point d'arrive axe x
        let mut x_end = self.position.0 + lenth as u16;
        if x_end < graph.max_lenth {
            x_end += 1u16;
        }

        // point de départ axe y
        let mut y_start = self.position.1;
        if y_start > 0 {
            y_start -= 1u16;
        }

        // point de d'arrivé axe y
        let mut y_end = self.position.1 + 1u16;
        if y_end > graph.max_index_line   {
            y_end = self.position.1;
        }

        for y in y_start..y_end + 1u16 {
            
            let result = graph.explore_line(Point(x_start,y), Point(x_end,y), Type::Symbol(".".to_string()));
            if result == true {
              //  println!("la node {:?} possède un symbole en voisin ",self.node_type.get_integer_value());
                return true;
            }
        }
        return false

    }

    
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub max_lenth: u16,
    pub max_index_line: u16,
}

impl Graph {

    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            max_lenth: 0,
            max_index_line: 0,
        }   
    }

    pub fn set_max_lenth(&mut self,lenth: u16) {
        self.max_lenth = lenth;
    }

    pub fn set_max_index_line(&mut self,lenth: u16) {
        self.max_index_line = lenth;
    }

    pub fn add_node(&mut self,node: Node) {
        self.nodes.push(node);
    }

    pub fn find_by_type(&self,node_type: Type) -> Vec<&Node> {
        self.nodes
        .iter()
        .filter(|&node| match &node.node_type {
            Type::PartNumber(_) => { 
              //  println!("node de type partnumber {:?} ", node);
                matches!(node_type, Type::PartNumber(_))
            },
            Type::Symbol(_) => {
              //  println!("node de type symbolre {:?} ", node);
                matches!(node_type, Type::Symbol(_))
            },
            Type::Other(_) => { 
             //   println!("node de type other {:?} ", node);
                matches!(node_type, Type::Other(_))
            },
        })
        .filter(|node| {

            node.is_real_part_number(self) == true
        })
        .collect()
    }

    pub fn find_note_at(&self, point: Point) -> Option<&Node> {
        self.nodes.iter().find(|&node| {
            node.position == point
        })
    }

    pub fn explore_line(&self,start_point: Point, end_point: Point, type_to_check: Type) -> bool {
        for x in start_point.0..end_point.0 + 1u16 {
            
            let node = self.find_note_at(Point(x,start_point.1));
            match node {
                Some(node) => {
                   
                    if matches!(node.node_type,Type::Symbol(_)) {
                        return true;
                    }
                },
                None => continue,
            }
        }
        return false;
    }
}