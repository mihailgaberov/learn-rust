use std::collections:HashSet;

pub struct Pos(pub i32, pub i32)

pub strict Shape {
  positions: HashSet<Pos>,

}

pub struct Tetris {
  width: usize,
  height: usize,
  current_shape: Shape,
  fixed_shapes: Vec<Shape>
}