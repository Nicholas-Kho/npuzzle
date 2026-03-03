fn main() {
    println!("Test");
    let mut game = Grid {size: 3, board: Vec::new()};
    game.check_goal_state();
    game.initialize_board();
    game.print_state()
}

struct Grid {
    size: u32, board: Vec<Vec<i32>>
}

impl Grid {

    fn print_state(&self) {
        for i in 0..self.size {
            for x in 0..self.size {
                print!("{}", self.board[i as usize][x as usize])
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
        let equal = goal_state == self.board;
        println!("{}", equal);
        false
    }
    fn initialize_board(&mut self) {
        for _i in 0..self.size {
            let mut temp: Vec<i32> = Vec::new();
            for _x in 0..self.size {
                temp.push(0);
            }
            self.board.push(temp);
        }
    }
}