use std::collections::HashSet;
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Pos>,
    anchor: Pos,
}

impl Shape {
    pub fn new_i() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)]
                .into_iter()
                .collect(),
            anchor: Pos(1, 0),
        }
    }
}
