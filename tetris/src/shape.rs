use std::collections::HashSet;
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Pos>,
    anchor: Pos,
}

macro_rules! impl_shape_constructor {
    ($( $new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self {
                Self {
                    positions: [$( $pos ),*].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )*
    };
}

impl Shape {
    impl_shape_constructor! {
        new_i: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] anchored at Pos(1, 0);
        new_o: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] anchored at Pos(0, 0);
    }
}
