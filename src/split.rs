use std::boxed::Box;

enum Orientation{
    Vertical,
    Horizontal,
}

pub struct Split{
    dir: Orientation,
    v_child: Box<Split>,
    h_child: Box<Split>,
}
