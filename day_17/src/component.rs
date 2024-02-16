use std::cmp::Ordering;



#[derive(Debug,Clone,Eq,PartialEq,Hash)]
pub struct Bloc {
    pub x: usize,
    pub y: usize,
    pub score: u32,
    pub color: String,
    pub h_score: usize,
    pub f_score: usize,
    pub neighbors: Vec<(usize,usize)>
}


// pour le trie dans la file de l'algo
impl Ord for Bloc {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Bloc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Bloc {
    
    pub fn new(x: usize, y:usize, score: u32) -> Bloc {
        Bloc {
            x,
            y,
            score,
            color: "WHITE".to_string(),
            h_score: 0,
            f_score: 0,
            neighbors: Vec::new()
        }
    }
}