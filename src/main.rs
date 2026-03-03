fn main() {
    let mut game = Grid {size: 4, board: Vec::new()};
    game.check_goal_state();
    game.initialize_board();
    game.print_state()
}

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
    fn get_number_index(&self, num: i32) -> Coord {
        for i in 0..self.size {
            for j in 0..self.size {
                if (self.board[i as usize][j as usize] == num) {
                    let test : Coord = Coord { x: i as i32, y: j as i32};
                    return test;
                }
            }
        }

    }
    fn get_movable_tiles(&self) -> Vec<i32> {
        ;
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