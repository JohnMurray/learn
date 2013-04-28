trait Times {
    fn times(&self, op : &fn() -> bool);
}

impl Times for int {
    fn times(&self, op : &fn() -> bool) {
        let mut n = 0;
        while n < *self {
            if !op() {break; }
            n += 1;
        }
    }
}

struct World {
    bottom_left: Option<~GridBlock>
}
struct GridBlock {
    top :    Option<~GridBlock>,
    right:   Option<~GridBlock>,
    bottom:  Option<~GridBlock>,
    left:    Option<~GridBlock>
}



impl World {
    fn new(rows : int, columns : int) -> ~World {
        let mut curr_row : Option<~GridBlock> = None;
        for rows.times() {
            if curr_row == None {
                curr_row = GridBlock::new_empty();
            }
            else {

            }
            for columns.times() {

            }
        }
        ~World { bottom_left: None }
    }
}


impl GridBlock {
    fn new(top    : Option<~GridBlock>, 
           right  : Option<~GridBlock>,
           bottom : Option<~GridBlock>, 
           left   : Option<~GridBlock>) -> Option<~GridBlock> {
        Some(~GridBlock {
            top    : top,
            right  : right,
            bottom : bottom,
            left   : left
        })
    }

    fn new_empty() -> Option<~GridBlock> {
        Some(~GridBlock { top: None, right: None, bottom: None, left: None })
    }
}




fn main() -> () {
    let world = World::new(8, 7);
}