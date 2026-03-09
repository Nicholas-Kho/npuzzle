use std::io;
use std::num::ParseIntError;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

fn main() {
    let mut game = Grid {size: 4, board: Vec::new(), moves: 0, iterations: 1, heuristic: 1};
    game.check_goal_state();
    game.initialize_board();
    //game.board = vec![
    //    vec![2, 8, 3],
    //    vec![1, 6, 4],
    //    vec![7, 0, 5],
    //];
    game.print_state();
    //game.print_state();

    //let to_index: i32 = 0;
    //let index = game.get_number_index(to_index);
    //match index {
    //    Some(c) => println!("Index of {}: {},{}", to_index, c.x(), c.y()),
    //    None => {println!("No index found")}
    //}
    //println!("Game is solvable: {}", game.is_solvable());

    let mut optimal : Vec<i32> = Vec::new();
    //optimal = astar(game.clone());

    let mut solved: bool = false;
    while !solved {
        println!("Optimal moves: {:?}", optimal);
        println!("Inversions: {} ", game.calculate_inversions_any(game.board.clone()));
        game.print_stats();
        let movables: Vec<Coord> = game.get_movable_tiles();
        game.check_goal_state();
        game.print_state();
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
    println!("Puzzle has been solved!")
}


fn astar(g: Grid) -> Vec<i32> {
    println!("Running A*..");
    let mut history: Vec<Node> = Vec::new();
    let mut fringe: PriorityQueue<Node, Reverse<i32>> = explore(Node {c: g, moves: Vec::new()}, &history);
    history.extend(get_keys(fringe.clone()));

    while !fringe.is_empty() {
        let node = fringe.pop().unwrap();
        //println!("F: {}", node.1.0);
        fringe.extend(explore(node.0.clone(), &history));
        history.push(node.0.clone());
        //println!("{:?}", get_keys(fringe.clone()));
        if node.0.c.check_goal_state() {
            //println!("Solution found: {:?}", history);
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

fn explore(n: Node, history: &Vec<Node>) -> PriorityQueue<Node, Reverse<i32>> {
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
            fringe.push(n, Reverse(f));
        }


    }
    fringe
}



fn remove_option(l: Vec<Option<i32>>) -> Vec<i32> {
    let mut new: Vec<i32> = Vec::new();
    for value in &l {
        match value {
            Some(num) => {new.push(*num);},
            None => {}
        }
    }
    new
}




#[derive(Hash, Debug, Clone)]
struct Node {
    c: Grid, moves: Vec<i32>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {

        if (self.c.board == other.c.board) {
            //println!("equal {:?} {:?}", self.c.board, other.c.board);
            return true
        }
        false
    }
}
impl Eq for Node {}


#[derive(Clone)]
#[derive(Debug)] // Makes printing debug possible {:?}
#[derive(PartialEq)] // Makes comparisons (==) possible
struct Coord {
    x: i32, y: i32
}


impl Coord {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Grid {
    size: u32,
    board: Vec<Vec<i32>>,
    moves: i32,
    iterations: i32,
    heuristic: i32
}

impl Grid {
    fn print_stats(&self) {
        let mut text: String = String::from("");
        let h : i32;

        text.push_str("Heuristic Type: ");
        match self.heuristic {
            0 => {text.push_str("Misplaced Tiles"); h = self.misplace_heuristic()},
            1 => {text.push_str("Manhattan"); h = self.manhattan_heuristic()},
            _ => {text.push_str("Unknown heuristic"); h = 0}
        }
        text.push_str(" | f(n) = ");
        text.push_str(self.f().to_string().as_str());
        text.push_str(" | g(n) = ");
        text.push_str(self.g().to_string().as_str());
        text.push_str(" | h(n) = ");
        text.push_str(h.to_string().as_str());

        println!("{}", text);
    }
    fn is_solvable(&self) -> bool {

        let is_even: bool = self.size % 2 == 0;
        let inversions: i32 = self.calculate_inversions_any(self.board.clone());
        //println!("Number of inversions: {}", inversions);
        if (self.size == 3) {
            return inversions % 2 == 1;
        }
        if !is_even {
            return inversions % 2 == 0;
        } else if is_even {
            let blank_index: Option<Coord> = self.get_number_index(0);
            match blank_index {
                Some(index) => {
                    // Check if blank is even starting from the bottom
                    let blank_even_index: bool = (index.y() + 1 + (index.x()%2)) % 2 == 0;
                    let row_from_bottom: i32 = (self.size - index.x() as u32) as i32;
                    //println!("Is blank even: {}", blank_even_index);
                    //if (blank_even_index && inversions % 2 == 0) {return true}
                    //if (!blank_even_index && inversions % 2 == 1) {return true}
                    return (inversions + row_from_bottom) % 2 == 0;
                }
                None => {}
            }
        }
        false
    }

    fn calculate_inversions_any(&self, grid: Vec<Vec<i32>>) -> i32 {
        let mut inversions: i32 = 0;
        for i in 0..self.size*self.size {
            let num: i32 = grid[(i/self.size) as usize][(i % self.size) as usize];
            for x in i+1..self.size*self.size {
                let comparison: i32 = grid[(x/self.size) as usize][(x % self.size) as usize];
                if (num != 0 && comparison != 0 && num > comparison) {
                    inversions += 1
                }
            }
        }
        inversions
    }

    fn g(&self) -> i32 {
        self.moves
    }

    fn f(&self) -> i32 {
        match self.heuristic {
            0 => self.misplace_heuristic() + self.g(),
            1 => self.manhattan_heuristic() + self.g(),
            _ => {println!("Invalid heuristic value chosen"); 0}
        }
    }

    fn misplace_heuristic(&self) -> i32 {
        let mut misplaced: i32 = 0;
        let mut goal_state : Vec<Vec<i32>> = self.generate_goal_state();
        for i in 0..(self.size*self.size) {
            if self.board[(i / self.size) as usize][(i % self.size) as usize] != goal_state[(i / self.size) as usize][(i % self.size) as usize] {
                misplaced += 1;
            }
        }
        misplaced
    }

    fn manhattan_heuristic(&self) -> i32 {
        let mut manhattan: i32 = 0;
        let goal_state = self.generate_goal_state();

        for i in 0..self.size {
            for x in 0..self.size {
                let c: Coord = self.get_number_index(goal_state[i as usize][x as usize]).unwrap();
                manhattan += (c.x - i as i32).abs() + (c.y - x as i32).abs()
            }
        }
        manhattan
    }

    fn set_tile(&mut self, coord: Coord, value: i32) {
        self.board[coord.x() as usize][coord.y() as usize] = value;
    }

    fn move_tile(&mut self, coord: Coord) {
        let temp = self.get_tile(&coord);
        let value: i32;
        match temp {
            Some(v) => {value = v;},
            None => {value = 0}
        }
        if self.get_tile(&Coord {x:coord.x()-1, y:coord.y()}) == Some(0) {
            self.set_tile(Coord {x:coord.x()-1, y:coord.y()}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
            self.moves += 1;
            return
        }
        if self.get_tile(&Coord {x:coord.x(), y:coord.y()-1}) == Some(0) {
            self.set_tile(Coord {x:coord.x(), y:coord.y()-1}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
            self.moves += 1;
            return
        }
        if self.get_tile(&Coord {x:coord.x(), y:coord.y()+1}) == Some(0) {
            self.set_tile(Coord {x:coord.x(), y:coord.y()+1}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
            self.moves += 1;
            return
        }
        if self.get_tile(&Coord {x:coord.x()+1, y:coord.y()}) == Some(0) {
            self.set_tile(Coord {x:coord.x()+1, y:coord.y()}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
            self.moves += 1;
            return
        }
    }
    fn get_number_index(&self, num: i32) -> Option<Coord> {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.board[i as usize][j as usize] == num {
                    let test : Coord = Coord { x: i as i32, y: j as i32};
                    return Some(test);
                }
            }
        }
        None
    }

    fn get_tile(&self, coord: &Coord) -> Option<i32> {
        let x: i32 = coord.x();
        let y: i32 = coord.y();
        if x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
            return Some(self.board[x as usize][y as usize])
        }
        None
    }
    fn get_movable_tiles(&self) -> Vec<Coord> {
        let empty_tile: Option<Coord> = self.get_number_index(0);
        if let Some(coord) = empty_tile {
            let x: i32 = coord.x();
            let y: i32 = coord.y();
            let mut quad: Vec<Coord> = Vec::new();
            if x > 0 {quad.push(Coord { x: x-1, y: y });}
            if (y > 0) {quad.push(Coord { x: x, y: y-1});}
            if self.size > (y+1) as u32 {quad.push(Coord { x: x, y: y+1 });}
            if self.size > (x+1) as u32 {quad.push(Coord { x: x+1, y: y });}
            quad
        } else {
            Vec::new()
        }

    }
    fn print_state(&self) {
        for i in 0..self.size {
            for x in 0..self.size {
                if (self.board[i as usize][x as usize]) > 9 {
                    print!("{} ", self.board[i as usize][x as usize])
                } else {
                    print!("{}  ", self.board[i as usize][x as usize])
                }
            }
            print!("\n")
        }
    }

    fn generate_goal_state(&self) -> Vec<Vec<i32>> {

        // Assignment goal state:
        if (self.size == 3) {
            return vec![
                vec![1, 2, 3],
                vec![8, 0, 4],
                vec![7, 6, 5],
            ];
        }

        let mut counter: i32 = 0;
        let mut goal_state : Vec<Vec<i32>> = Vec::new();

        // Populate an empty grid of size.size * size.size
        for _i in 0..self.size {
            let mut temp: Vec<i32> = Vec::new();
            for _x in 0..self.size {
                temp.push(0);
            }
            goal_state.push(temp);
        }
        let n: usize = (self.size * self.size) as usize;


        // Generate the grid
        if (self.size == 3) {counter += 1}
        for mut i in 0..n {
            goal_state[i / self.size as usize][i % self.size as usize] = counter;
            counter += 1;
            if (self.size == 3 && i == 3) {
                counter = 0
            }
            if (self.size == 3 && i == 4) {
                counter = 5
            }
        }
        goal_state
    }
    fn check_goal_state(&self) -> bool {
        let mut goal_state : Vec<Vec<i32>> = self.generate_goal_state();
        goal_state == self.board
    }
    fn initialize_board(&mut self) {
        self.board.clear();
        let mut numbers: Vec<i32> = Vec::new();
        for i in 0..self.size*self.size {
            numbers.push(i as i32);
        }

        for _i in 0..self.size {
            let mut temp: Vec<i32> = Vec::new();
            for _x in 0..self.size {
                temp.push(numbers.remove(rand::random_range(0..numbers.len())));
            }
            self.board.push(temp);
        }
        if (!self.is_solvable()) {
            self.iterations += 1;
            println!("Generated an unsolvable board, regenerating");
            self.initialize_board()
        } else {
            println!("Found a solvable game! Number of grids checked: {}", self.iterations)
        }
    }
}