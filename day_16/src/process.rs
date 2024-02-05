use std::collections::{HashSet, VecDeque};

use array2d::Array2D;

use crate::component::{Point, Position, Direction};



pub fn process_part_one(input: &Array2D<char>){

    let mut visited_node: HashSet<Position> = HashSet::new();
    let mut seen_node: HashSet<Point> = HashSet::new();

    // init de la pile
    let mut move_stack: VecDeque<Point> = VecDeque::new();


    let current_point = get_current_point(input,(0,0), Direction::Right);

    move_stack.push_back(current_point.clone());
    seen_node.insert(current_point);

    while let Some(node) = move_stack.pop_front() {

        let next_node = make_signal_path(node, input, &mut visited_node);
    }
}

fn get_current_point(map: &Array2D<char>,coordonnee: (usize,usize),direction: Direction) -> Point {
    Point {
        x: coordonnee.0,
        y: coordonnee.1,
        c: map.get(0, 0).unwrap().to_ascii_lowercase(),
        direction: direction
    }
}

fn make_signal_path(position: Point, map: &Array2D<char>, visited_node: &mut HashSet<Position>) -> Vec<Point> {
    
    let current_point = get_current_point(map, (position.x,position.y), position.direction);

    let mut seen_node: HashSet<Point> = HashSet::new();
    let mut prev_len = seen_node.len();

    loop {
        visited_node.insert(Position {x: position.x, y: position.y});

    }
}

fn move_signal(point: Point, map: &Array2D<char>){
    let Point {x,y,c,direction} = point;



    match c {
        '.' => match direction {
            Direction::Right =>,
            Direction::Left =>,
            Direction::Up =>,
            Direction::Down,
        }
    }
}