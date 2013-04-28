

struct Vector(float,float);

fn main() {
    let x: ~int = ~1;
    let y: ~int = inc(x);
    io::println(fmt!("%?", y));

    let _45 = angle(Vector(0.5,0.5));
    io::println(fmt!("radians %?", _45));
    io::println(fmt!("degrees %?", radians_to_degrees(_45)));
}


fn angle(vector: Vector) -> float {
    let pi = float::consts::pi;
    match vector {
        Vector(0f, y) if y < 0f => { 1.5 * pi }
        Vector(0f, _) => { 0.5 * pi }
        Vector(x,y) => { float::atan(y/x) }
    }
}


fn radians_to_degrees(radians : float) -> float {
    static conversion_factor :float = 360f / float::consts::pi;
    radians * conversion_factor
}


fn inc(x: &int) -> ~int {
    ~((*x) + 1)
}
