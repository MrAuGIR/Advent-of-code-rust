
#[derive(Debug, PartialEq, Clone)]
pub enum Tiles {
    VerticalPipe,
    HorizontalPipe,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    Ground,
    Starting
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node
{
    pub x: usize,
    pub y: usize,
    pub tile: Tiles,
    pub neighbors: Vec<(usize, usize)>,
    pub visited: bool,
}

impl Node {

    pub fn new(x: usize, y: usize, tile: Tiles) -> Node {
        let neighbors = Vec::new();
        Node {
            x,y,tile, neighbors, visited: false
        }
    }
}

#[derive(Debug,Clone)]
pub struct Map{
    pub map: Vec<Vec<Node>>,
    pub start_node: Option<Node>
}

impl Map{

    pub fn new() -> Map{
        Map { map: Vec::new(), start_node: None }
    }

    pub fn set_start_node(&mut self, node: Node)
    {
        self.start_node = Some(node);
    }


    pub fn parcour_en_profondeur(&mut self) -> Option<Vec<Vec<Node>>>{
        if let Some(start_node) = &self.start_node {
            let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
            let mut path = vec![];
            let mut paths: Vec<Vec<Node>> = vec![];
            let mut loop_found = false; 

            self.dfs(
                &start_node.x, 
                &start_node.y, 
                &mut visited, 
                &mut path, 
                None,
                 &mut paths,
                 &mut loop_found
                )
                ;

            return Some(paths);
        } else {
            return None;
        }
    }

    fn dfs(&self, x: &usize, y: &usize, visited: &mut Vec<Vec<bool>>, path: &mut Vec<Node>, prev_node: Option<Node>, paths: &mut Vec<Vec<Node>>, loop_found: &mut bool ) {
        if *x >= self.map[0].len() || *y >= self.map.len() || visited[*y][*x] {
            return;
        }

        let node = &self.map[*y][*x];
        visited[*y][*x] = true;
        path.push(node.clone());

        
        let prev_node = node.clone();
        
        // Continue DFS in all directions
        'neighbors:for (x,y) in &node.neighbors {
            
            // on recupère la node 
            let tmp_node = self.map.get(*y).unwrap().get(*x).unwrap();

            if *loop_found {
                break; // Stop la boucle 'neighbors
            }
            
            match tmp_node.tile {
                Tiles::Starting => {
                    if prev_node.tile != Tiles::Starting {
                        *loop_found = true;
                        paths.push(path.clone());
                        break 'neighbors;
                    }
                },
                Tiles::Ground => continue,
                _ => self.dfs(x, y, visited, path,Some(prev_node.clone()),paths,loop_found),
            };
        }

        // Backtrack
        path.pop();
     //   visited[*y][*x] = false;
    }
}


