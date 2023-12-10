



#[derive(Debug,PartialEq, Clone)]
pub enum Type {
    Symbol(String),
    PartNumber(u32),
    Gear(String),
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

#[derive(Debug,PartialEq,Clone)]
pub struct Point(pub u16,pub u16);

#[derive(Debug, Clone)]
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

    pub fn sum_real_gear(&self, graph: &Graph) -> u32 {
        let result = self.node_part_number_in_line(graph);

        println!("longeur result {:?}",result.len());
        for node in &result {
            if result.len() == 2 {
                println!("{:#?}", node.node_type.get_integer_value().unwrap_or(0));
            }
        }
        if result.len() != 2 {
            return 0u32;
        }

        let value = result.iter().map(|n| n.node_type.get_integer_value().unwrap() as u32).product();
        println!("---------------------");
        return value;
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

    fn node_part_number_in_line(&self, graph: &Graph) -> Vec<Node> {
        let lenth = 0;
        
        let mut result: Vec<Node> = Vec::new();
        // point de départ axe x
        let mut x_start = self.position.0;
        if self.position.0 > 3 {
            x_start -= 3u16;
        } else {
            x_start = 0;
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
            
            result.extend(graph.get_part_number_linked(Point(x_start,y), Point(x_end,y), &self.position));
            
            //  println!("la node {:?} possède un symbole en voisin ",self.node_type.get_integer_value());
        }
        return result;
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
            Type::Gear(_) => {
                // println!("node de type gear {:?} ", node);
                matches!(node_type, Type::Gear(_))
            }
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

    pub fn find_gear(&self,node_type:Type) -> Vec<&Node> {
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
            Type::Gear(_) => {
                // println!("node de type gear {:?} ", node);
                matches!(node_type, Type::Gear(_))
            }
        })
        .collect()
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

    pub fn get_part_number_linked(&self,start_point: Point, end_point: Point, origin_point: &Point) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        
        for x in start_point.0..end_point.0 + 1u16 {
            
            let node = self.find_note_at(Point(x,start_point.1));
            match node {
                Some(node) => {
                   
                    if matches!(node.node_type,Type::PartNumber(_)) {

                        let mut origin_x = origin_point.0;
                        if origin_point.0 > 0 {
                            origin_x -= 1u16;
                        };

                        if node.position.0 + node.node_type.get_integer_value().unwrap().to_string().len()  as u16 >= origin_x + 1 {
                            nodes.push(node.clone());
                        } 
                       
                    }
                },
                None => continue,
            }
        }
        return nodes;
    }
}