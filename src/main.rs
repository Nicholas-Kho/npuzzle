fn main() {
    let game = Grid {size: 3, board: Vec::new()};
    game.check_goal_state();
}

struct Grid {
    size: u32, board: Vec<Vec<i32>>
}

impl Grid {
    fn check_goal_state(&self) -> bool {
        let counter: i32 = 0;
        let mut goal_state : Vec<Vec<i32>> = Vec::new();
        let n: usize = (self.size * self.size) as usize;
        for i in 0..=n {
            goal_state[i / 3][i % 3] = counter
        }
        println!("{:?}", goal_state);
        return false
    }
    fn initialize_board() {
        let _board = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
    }
}