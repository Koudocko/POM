use std::boxed::Box;

pub enum Orientation{
    Vertical,
    Horizontal,
}

type SplitChild = Option<Box<Split>>;

pub struct Split{
    dir: Orientation,
    v_child: SplitChild,
    h_child: SplitChild,
    size: u8,
}

impl Split{
    pub fn new(dir: Orientation, v_child: SplitChild, h_child: SplitChild, size: u8)-> Self{
        Split{ dir, v_child, h_child, size }
    }
}

impl Default for Split{
    fn default()-> Self{
        Split{ 
            dir: Orientation::Vertical, 
            v_child: None,
            h_child: None,
            size: 100,
        }
    }
}
