mod split;

use split::*;

fn main(){
    let mut windows = vec![Split::default()];
    windows[0].insert_split(Split::new(None, None, 50), Orientation::Vertical);
    windows[0].insert_split(Split::new(None, None, 75), Orientation::Horizontal);
    windows[0].display_splits();
}
