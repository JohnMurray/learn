

fn main() -> () {
  let mut x = @mut 5;

  *x += 5;

  io::println(int::to_str(*x));
}
