fn main() {
    let mut game = Grid {size: 3, board: Vec::new()};
    game.check_goal_state();
    game.initialize_board();
    game.print_state();

    let to_index: i32 = 3;
    let index = game.get_number_index(to_index);
    match index {
        Some(c) => println!("Index of {}: {},{}", to_index, c.x(), c.y()),
        None => {println!("No index found")}
    }

    let movables: Option<Vec<Coord>> = game.get_movable_tiles();
    if let Some(m) = movables {
        println!("Movables {:?}", m);
        //for i in 0..=4 {
        //    print!("{:?}, ", game.get_tile(m.get(0)));
        //}
        for element in &m {
            println!("{:?}", game.get_tile(element));
        }
    }
    else {
        println!("No movables found");
    }
}

#[derive(Debug)]
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
    size: u32, board: Vec<Vec<i32>>
}

impl Grid {
    fn get_number_index(&self, num: i32) -> Option<Coord> {
        for i in 0..self.size {
            for j in 0..self.size {
                if (self.board[i as usize][j as usize] == num) {
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
        if (x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32) {
            return Some(self.board[x as usize][y as usize])
        }
        None
    }
    fn get_movable_tiles(&self) -> Option<Vec<Coord>> {
        let empty_tile: Option<Coord> = self.get_number_index(0);
        if let Some(coord) = empty_tile {
            let x: i32 = coord.x();
            let y: i32 = coord.y();
            let mut quad: Vec<Coord> = Vec::new();
            quad.push(Coord { x: x-1, y: y });
            quad.push(Coord { x: x, y: y-1});
            quad.push(Coord { x: x, y: y+1 });
            quad.push(Coord { x: x+1, y: y });
            return Some(quad);

            return None
        } else {
            None
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
        for i in 0..n {
            goal_state[i / self.size as usize][i % self.size as usize] = counter;
            counter += 1;
        }
        println!("{:?}", goal_state);
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