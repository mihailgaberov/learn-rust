use std::collections::HashSet;
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);
#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Pos>,
}
pub struct Tetris {
    width: usize,
    height: usize,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
}

impl Tetris {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            current_shape: todo!(),
            fixed_shapes: vec![],
        }
    }
}
