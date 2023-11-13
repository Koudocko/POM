use std::rc::Rc;

#[derive(PartialEq)]
pub enum Orientation{
    Vertical,
    Horizontal,
}

type SplitChild = Option<Rc<Split>>;

pub struct Split{
    v_child: SplitChild,
    h_child: SplitChild,
    size: u8,
}

impl Split{
    pub fn new(v_child: SplitChild, h_child: SplitChild, size: u8)-> Self{
        Split{ v_child, h_child, size }
    }

    pub fn display_splits(&self){
        println!("{}", self.size);

        let mut next_split = &self.v_child;
        if next_split.is_some(){
            next_split.as_ref().unwrap().display_splits();
        }

        next_split = &self.h_child;
        if next_split.is_some(){
            next_split.as_ref().unwrap().display_splits();
        }
    }

    pub fn insert_split(&mut self, mut split: Split, pos: Orientation){
        if pos == Orientation::Vertical{
            if let Some(v_child) = &self.v_child{
                let children = Rc::clone(v_child);

                split.v_child = Some(children);
                self.v_child = Some(Rc::new(split));
            }
            else{
                self.v_child = Some(Rc::new(split));
            }
        }
        else{
            if let Some(h_child) = &self.h_child{
                let children = Rc::clone(h_child);

                split.h_child = Some(children);
                self.h_child = Some(Rc::new(split));

            }
            else{
                self.h_child = Some(Rc::new(split));
            }
        }
    }
}

impl Default for Split{
    fn default()-> Self{
        Split{ 
            v_child: None,
            h_child: None,
            size: 100,
        }
    }
}
