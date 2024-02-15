use std::collections::VecDeque;

use array2d::Array2D;

use crate::component::{Bloc,Direction};



 pub fn parcours_aStar(map: Array2D<u32>) {

    let mut queue: VecDeque<Bloc> = VecDeque::new();



    while let Some(current_bloc) = queue.pop_front() {

    }

 }
/* 
 fn get_neighbors(bloc: Bloc, map: Array2D<u32>) -> Vec<Bloc> {

    let x_sub = bloc.x.checked_sub(1);
    let y_sub = bloc.y.checked_sub(1);
    let x_add = bloc.x.checked_add(1);
    let y_add = bloc.y.checked_add(1);

    let neighbor_left = x_sub.
        and_then(|x| map.get(bloc.y,x))
        .map(|u| Bloc {x: bloc.x - 1, y:bloc.y,lost:*u,color: "white".to_string()});

    let neighbor_top = y_sub
        .and_then(|y| map.get(y,bloc.x))
        .map(|u| Bloc {x: bloc.x, y: bloc.y - 1, lost: *u, color: "white".to_string()});

    let neighbor_right = map.get(bloc.x,bloc.y + 1).map(|u| Bloc {x: bloc.x, y: bloc.y +1,lost: *u, color: "White".to_string()});
 }*/