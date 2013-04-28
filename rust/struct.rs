

struct Bob {
    priv name : ~str,
    priv age : int
}

impl Bob {
    fn new(name: ~str, age: int) -> Bob {
        Bob { name: name, age: age }
    }

    fn get_older(&mut self) -> () {
        self.age += 1;
    }
}


fn main() -> () {
    let mut bob = Bob::new(~"Roberto", 35);
}