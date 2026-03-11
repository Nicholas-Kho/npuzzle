use crate::Coord;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Grid {
    pub(crate) size: u32,
    pub(crate) board: Vec<Vec<i32>>,
    pub(crate) moves: i32,
    pub(crate) iterations: i32,
    pub(crate) heuristic: i32
}

impl Grid {
    pub(crate) fn print_stats(&self) {
        let mut text: String = String::from("");
        let h: i32;

        text.push_str("Heuristic Type: ");
        match self.heuristic {
            0 => {
                text.push_str("Misplaced Tiles");
                h = self.misplace_heuristic()
            },
            1 => {
                text.push_str("Manhattan (weighted by n^2)");
                h = self.manhattan_heuristic()
            },
            _ => {
                text.push_str("Unknown heuristic");
                h = 0
            }
        }
        text.push_str(" | f(n) = ");
        text.push_str(self.f().to_string().as_str());
        text.push_str(" | g(n) = ");
        text.push_str(self.g().to_string().as_str());
        text.push_str(" | h(n) = ");
        text.push_str(h.to_string().as_str());
        if self.heuristic == 1 {
            text.push_str(" (x3) ");
        }

        println!("{}", text);
    }
    fn is_solvable(&self) -> bool {
        let is_even: bool = self.size % 2 == 0;
        let inversions: i32 = self.calculate_inversions_any(self.board.clone());
        //println!("Number of inversions: {}", inversions);
        if self.size == 3 {
            return inversions % 2 == 1;
        }
        if !is_even {
            return inversions % 2 == 0;
        } else if is_even {
            let blank_index: Option<Coord> = self.get_number_index(0);
            match blank_index {
                Some(index) => {
                    // Check if blank is even starting from the bottom
                    //let blank_even_index: bool = (index.y() + 1 + (index.x() % 2)) % 2 == 0;
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

    pub(crate) fn calculate_inversions_any(&self, grid: Vec<Vec<i32>>) -> i32 {
        let mut inversions: i32 = 0;
        for i in 0..self.size * self.size {
            let num: i32 = grid[(i / self.size) as usize][(i % self.size) as usize];
            for x in i + 1..self.size * self.size {
                let comparison: i32 = grid[(x / self.size) as usize][(x % self.size) as usize];
                if num != 0 && comparison != 0 && num > comparison {
                    inversions += 1
                }
            }
        }
        inversions
    }

    fn g(&self) -> i32 {
        self.moves
    }

    pub(crate) fn f(&self) -> i32 {
        match self.heuristic {
            0 => self.misplace_heuristic() + self.g(),
            1 => self.manhattan_heuristic() * (self.size) as i32 + self.g(),
            _ => {
                println!("Invalid heuristic value chosen");
                0
            }
        }
    }

    pub(crate) fn misplace_heuristic(&self) -> i32 {
        let mut misplaced: i32 = 0;
        let goal_state: Vec<Vec<i32>> = self.generate_goal_state();
        for i in 0..(self.size * self.size) {
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

    pub(crate) fn move_tile(&mut self, coord: Coord) {
        let temp = self.get_tile(&coord);
        let value: i32;
        match temp {
            Some(v) => { value = v; },
            None => { value = 0 }
        }
        if self.get_tile(&Coord { x: coord.x() - 1, y: coord.y() }) == Some(0) {
            self.set_tile(Coord { x: coord.x() - 1, y: coord.y() }, value);
            self.set_tile(Coord { x: coord.x(), y: coord.y() }, 0);
            self.moves += 1;
            return
        }
        if self.get_tile(&Coord { x: coord.x(), y: coord.y() - 1 }) == Some(0) {
            self.set_tile(Coord { x: coord.x(), y: coord.y() - 1 }, value);
            self.set_tile(Coord { x: coord.x(), y: coord.y() }, 0);
            self.moves += 1;
            return
        }
        if self.get_tile(&Coord { x: coord.x(), y: coord.y() + 1 }) == Some(0) {
            self.set_tile(Coord { x: coord.x(), y: coord.y() + 1 }, value);
            self.set_tile(Coord { x: coord.x(), y: coord.y() }, 0);
            self.moves += 1;
            return
        }
        if self.get_tile(&Coord { x: coord.x() + 1, y: coord.y() }) == Some(0) {
            self.set_tile(Coord { x: coord.x() + 1, y: coord.y() }, value);
            self.set_tile(Coord { x: coord.x(), y: coord.y() }, 0);
            self.moves += 1;
            return
        }
    }
    pub(crate) fn get_number_index(&self, num: i32) -> Option<Coord> {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.board[i as usize][j as usize] == num {
                    let test: Coord = Coord { x: i as i32, y: j as i32 };
                    return Some(test);
                }
            }
        }
        None
    }

    pub(crate) fn get_tile(&self, coord: &Coord) -> Option<i32> {
        let x: i32 = coord.x();
        let y: i32 = coord.y();
        if x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
            return Some(self.board[x as usize][y as usize])
        }
        None
    }
    pub(crate) fn get_movable_tiles(&self) -> Vec<Coord> {
        let empty_tile: Option<Coord> = self.get_number_index(0);
        if let Some(coord) = empty_tile {
            let x: i32 = coord.x();
            let y: i32 = coord.y();
            let mut quad: Vec<Coord> = Vec::new();
            if x > 0 { quad.push(Coord { x: x - 1, y: y }); }
            if y > 0 { quad.push(Coord { x: x, y: y - 1 }); }
            if self.size > (y + 1) as u32 { quad.push(Coord { x: x, y: y + 1 }); }
            if self.size > (x + 1) as u32 { quad.push(Coord { x: x + 1, y: y }); }
            quad
        } else {
            Vec::new()
        }
    }
    pub(crate) fn print_state(&self) {
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
        if self.size == 3 {
            return vec![
                vec![1, 2, 3],
                vec![8, 0, 4],
                vec![7, 6, 5],
            ];
        }

        let mut counter: i32 = 0;
        let mut goal_state: Vec<Vec<i32>> = Vec::new();

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
        if self.size == 3 { counter += 1 }
        for i in 0..n {
            goal_state[i / self.size as usize][i % self.size as usize] = counter;
            counter += 1;
            if self.size == 3 && i == 3 {
                counter = 0
            }
            if self.size == 3 && i == 4 {
                counter = 5
            }
        }
        goal_state
    }
    pub(crate) fn check_goal_state(&self) -> bool {
        let goal_state: Vec<Vec<i32>> = self.generate_goal_state();
        goal_state == self.board
    }
    pub(crate) fn initialize_board(&mut self) {
        self.board.clear();
        let mut numbers: Vec<i32> = Vec::new();
        for i in 0..self.size * self.size {
            numbers.push(i as i32);
        }

        for _i in 0..self.size {
            let mut temp: Vec<i32> = Vec::new();
            for _x in 0..self.size {
                temp.push(numbers.remove(rand::random_range(0..numbers.len())));
            }
            self.board.push(temp);
        }
        if !self.is_solvable() {
            self.iterations += 1;
            println!("Generated an unsolvable board, regenerating");
            self.initialize_board()
        } else {
            println!("Found a solvable game! Number of grids checked: {}", self.iterations)
        }
    }
}