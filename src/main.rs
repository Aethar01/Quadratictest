use rand::Rng;
#[derive(Debug, Copy, Clone)]
struct Quad {
    a: f32,
    b: f32,
    c: f32,
    r1: f32,
    r2: f32,
}

fn fill_quads() -> Vec<Quad> {
    let mut rng = rand::thread_rng();
    let mut quads: Vec<Quad> = Vec::new();
    for i in 0..usize::MAX {
        quads.insert(i, Quad {a: rng.gen_range(1.0..100.0), b: rng.gen_range(1.0..100.0), c: rng.gen_range(1.0..100.0), r1: 0.0, r2: 0.0})
    }
    return quads
}

fn quadratic_eq(mut quads: Quad) -> Quad {
        quads.r1 = -quads.b + ((quads.b * quads.b - 4.0 * quads.a * quads.c).sqrt()) / (2.0 * quads.a);    
        quads.r2 = -quads.b - ((quads.b * quads.b - 4.0 * quads.a * quads.c).sqrt()) / (2.0 * quads.a);
        return quads;
}

fn main() {
    use std::time::Instant;
    let a = fill_quads();
    let now = Instant::now();
    {
    let _b = a.iter().map(|x| quadratic_eq(*x)).collect::<Vec<Quad>>();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}
