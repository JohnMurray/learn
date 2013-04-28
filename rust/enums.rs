
struct Test(int);

fn main() -> () {
    let a = ~Test(3);
    let b : int = **a;
}