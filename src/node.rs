use crate::grid::Grid;

#[derive(Hash, Debug, Clone)]
pub(crate) struct Node {
    pub(crate) c: Grid, pub(crate) moves: Vec<i32>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {

        if self.c.board == other.c.board {
            //println!("equal {:?} {:?}", self.c.board, other.c.board);
            return true
        }
        false
    }
}
impl Eq for Node {}


