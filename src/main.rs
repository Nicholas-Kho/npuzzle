use std::io;
use std::num::ParseIntError;

fn main() {
    let mut game = Grid {size: 3, board: Vec::new(), moves: 0};
    game.check_goal_state();
    game.initialize_board();
    game.print_state();

    let to_index: i32 = 0;
    let index = game.get_number_index(to_index);
    match index {
        Some(c) => println!("Index of {}: {},{}", to_index, c.x(), c.y()),
        None => {println!("No index found")}
    }

    loop {
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
                            println!("Moved")
                        } else {
                            println!("Not movable")
                        }

                    },
                    None => println!("Not a valid number"),
                }
            }
            Err(Error) => {
                println!("Not a number, {}", Error);
            }
        }
    }


}

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

struct Grid {
    size: u32, board: Vec<Vec<i32>>, moves: i32
}

impl Grid {
    fn calculate_inversions(&self) -> i32 {
        let inversions: i32 = 0;
        for i in 0..self.size {
            for x in 0..self.size {
                for y in 0..(self.size**) {

                }
            }
        }
        inversions
    }
    fn calculate_g(&self) -> i32 {
        return self.moves;
    }

    fn calculate_heuristic(&self) {

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
            return
        }
        if self.get_tile(&Coord {x:coord.x(), y:coord.y()-1}) == Some(0) {
            self.set_tile(Coord {x:coord.x(), y:coord.y()-1}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
            return
        }
        if self.get_tile(&Coord {x:coord.x(), y:coord.y()+1}) == Some(0) {
            self.set_tile(Coord {x:coord.x(), y:coord.y()+1}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
            return
        }
        if self.get_tile(&Coord {x:coord.x()+1, y:coord.y()}) == Some(0) {
            self.set_tile(Coord {x:coord.x()+1, y:coord.y()}, value);
            self.set_tile(Coord {x:coord.x(), y:coord.y()}, 0);
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
            quad.push(Coord { x: x-1, y: y });
            quad.push(Coord { x: x, y: y-1});
            quad.push(Coord { x: x, y: y+1 });
            quad.push(Coord { x: x+1, y: y });
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
    fn check_goal_state(&self) -> bool {
        let mut counter: i32 = 0;
        let mut goal_state : Vec<Vec<i32>> = Vec::new();
        for _i in 0..self.size {
            let mut temp: Vec<i32> = Vec::new();
            for _x in 0..self.size {
                temp.push(0);
            }
            goal_state.push(temp);
        }
        let n: usize = (self.size * self.size) as usize;
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
        //println!("{:?}", goal_state);
        let _equal = goal_state == self.board;
        false
    }
    fn initialize_board(&mut self) {
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
    }
}