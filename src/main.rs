mod grid;
mod coord;
mod node;

use std::io;
use std::num::ParseIntError;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use crate::grid::Grid;
use crate::coord::Coord;
use crate::node::Node;

fn main() {
    let mut game = Grid {size: 3, board: Vec::new(), moves: 0, iterations: 1, heuristic: 1};
    game.check_goal_state();
    game.initialize_board();
    
    let mut set_state = false;

    // Change this to true if you wish for the initial state to be:
    // 2, 8, 3
    // 1, 6, 4
    // 7, 0, 5


    if set_state && game.size == 3{
        game.board = vec![
            vec![2, 8, 3],
            vec![1, 6, 4],
            vec![7, 0, 5],
        ];
    }

    game.print_state();

    //let mut test : PriorityQueue<String, Reverse<i32>> = PriorityQueue::new();
    //test.push("test".parse().unwrap(), Reverse(6));
    //test.push("testing".parse().unwrap(), Reverse(2));
    //test.push("testing123".parse().unwrap(), Reverse(130));
    //println!("{:?}", test.pop().unwrap());


    let mut optimal : Vec<i32> = Vec::new();
    println!("Inversions: {} ", game.calculate_inversions_any(game.board.clone()));

    // Heuristic takes too long for a 5x5 grid or anything bigger
    // Thus A* will not run on anything bigger than a 4x4
    // Game may still be played past a 4x4
    if game.size <= 4 {
        optimal = astar(game.clone());
    }


    let mut solved: bool = false;
    while !solved {
        println!("Optimal moves: {:?}", optimal);
        println!("Inversions: {} ", game.calculate_inversions_any(game.board.clone()));
        game.print_stats();
        let movables: Vec<Coord> = game.get_movable_tiles();
        game.check_goal_state();
        game.print_state();
        println!("Enter a number to move: ");
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed");
        //println!("test: {}", input_string);
        let number: Result<i32, ParseIntError> = input_string.trim().parse();
        //println!("{:?}", number);
        match number {
            Ok(i) => {
                let coord: Option<Coord> = game.get_number_index(i);
                match coord {
                    Some(c) => {
                        if movables.contains(&c) {
                            game.move_tile(c);
                            println!("Moves: {}", game.moves);


                            solved = game.check_goal_state();
                        } else {
                            println!("Not movable")
                        }
                    },
                    None => println!("Not a valid number"),
                }
            }
            Err(error) => {
                println!("Not a number, {}", error);
            }
        }
    }
    game.print_state();
    game.print_stats();
    println!("Puzzle has been solved!")
}


fn astar(g: Grid) -> Vec<i32> {
    println!("Running A*..");
    let mut explored_nodes = 0;
    let mut history: Vec<Node> = Vec::new();
    let mut fringe: PriorityQueue<Node, Reverse<i32>> = explore(Node {c: g, moves: Vec::new()}, &mut history);
    history.extend(get_keys(fringe.clone()));

    while !fringe.is_empty() {
        explored_nodes += 1;

        let node = fringe.pop().unwrap();
        //println!("Explored nodes: {} - f(n) = {}", explored_nodes, node.0.c.misplace_heuristic());
        //println!("F: {}", node.1.0);
        fringe.extend(explore(node.0.clone(), &mut history));
        history.push(node.0.clone());
        //println!("{:?}", get_keys(fringe.clone()));
        if node.0.c.check_goal_state() {
            //println!("Solution found: {:?}", history);
            println!("Explored nodes: {} - Optimal number of moves: {}", explored_nodes, node.0.moves.len());
            return node.0.moves
        }
    }
    println!("No solution found");
    Vec::new()
}

fn get_keys(mut fringe: PriorityQueue<Node, Reverse<i32>>) -> Vec<Node> {
    let mut keys: Vec<Node> = Vec::new();
    while let Some(item) = fringe.pop() {
        keys.push(item.0);
    }
    keys
}

fn explore(n: Node, history: &mut Vec<Node>) -> PriorityQueue<Node, Reverse<i32>> {
    let mut fringe: PriorityQueue<Node, Reverse<i32>> = PriorityQueue::new();
    let moves: Vec<Coord> = n.c.get_movable_tiles();
    for coord in &moves {

        // New grid
        let mut grid = n.c.clone();
        grid.move_tile(coord.clone());

        // Heuristic value
        let f = grid.f();

        // Append new move
        let mut m = n.moves.clone();
        m.push(n.c.get_tile(coord).unwrap());
        let n = Node {c: grid, moves: m};
        if !history.contains(&n) {
            fringe.push(n.clone(), Reverse(f));
            history.push(n);
        }


    }
    fringe
}
