#[derive(Clone)]
#[derive(Debug)] // Makes printing debug possible {:?}
#[derive(PartialEq)] // Makes comparisons (==) possible
pub(crate) struct Coord {
    pub(crate) x: i32, pub(crate) y: i32
}


impl Coord {
    pub(crate) fn x(&self) -> i32 {
        self.x
    }
    pub(crate) fn y(&self) -> i32 {
        self.y
    }
}