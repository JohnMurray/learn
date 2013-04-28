//extern mod core;
//use core::vec;

/*
trait Mappable<T> {
    fn map<U>(&self, &fn(&T) -> U) -> ~[U];
}

impl<'self,T> Mappable<T> for &'self [T] {
    fn map<U>(&self, mapper: &fn(&T) -> U) -> ~[U] {
        vec::map(*self, mapper)
    }
}
*/

fn main() -> () {
    // map tests
    let letters = ~[
        ~"a",
        ~"b",
        ~"c",
        ~"d"
    ];

    let letters2 = vec::map(letters, |&_| { 1 } );
    let letters3 = letters2.map( |&_| { 2 });

    for letters3.each |&name| {
        io::println(int::to_str(name));
    }


    // initialize an empty array
    let mut a : ~[int] = vec::with_capacity(20);

    // Can't do this
    // a[0] = 1;

    // can do this:
    for 20.times { a.push(1); }
    for a.each |&b| { io::println(int::to_str(b)); }


    // but... It's better to do this (to accomplish above)
    a = vec::from_elem(20, 1);
    for a.each |&b| { io::println(int::to_str(b)); }
}
