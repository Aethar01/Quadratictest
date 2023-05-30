use rand::Rng;
use rayon::prelude::*;
use num::complex::Complex;

const N: usize = 1000000;
const MAX_VALUE: f32 = 100.0;

#[derive(Debug, Copy, Clone)]
struct ComplexQuad {
    a: Complex<f32>,
    b: Complex<f32>,
    c: Complex<f32>,
    r1: Complex<f32>,
    r2: Complex<f32>,
}

#[derive(Debug, Copy, Clone)]
struct Quad {
    a: f32,
    b: f32,
    c: f32,
    r1: f32,
    r2: f32,
}

trait Quadratic<T> {
    fn new(a: T, b: T, c: T) -> Self;
    fn quadratic_eq(&mut self);
    fn get_roots(&self) -> (T, T);
    fn fill_quads(nvar: usize) -> Vec<Self> where Self: Sized;
}

impl Quadratic<Complex<f32>> for ComplexQuad {
    fn new(a: Complex<f32>, b: Complex<f32>, c: Complex<f32>) -> Self {
        Self { a, b, c, r1: Complex::new(0.0, 0.0), r2: Complex::new(0.0, 0.0) }
    }

    fn quadratic_eq(&mut self) {
        self.r1 = -self.b + (((self.b * self.b) - 4.0 * self.a * self.c)
                               .sqrt()) / (2.0 * self.a);    
        self.r2 = -self.b - (((self.b * self.b) - 4.0 * self.a * self.c)
                               .sqrt()) / (2.0 * self.a);
    }

    fn get_roots(&self) -> (Complex<f32>, Complex<f32>) {
        (self.r1, self.r2)
    }

    fn fill_quads(nvar: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut quads = Vec::with_capacity(N);
        for _ in 0..nvar {
            let a = Complex::new(rng.gen_range(-MAX_VALUE..=MAX_VALUE), 0.0);
            let b = Complex::new(rng.gen_range(-MAX_VALUE..=MAX_VALUE), 0.0);
            let c = Complex::new(rng.gen_range(-MAX_VALUE..=MAX_VALUE), 0.0);
            quads.push(ComplexQuad::new(a, b, c));
        }
        quads
    }
}

impl Quadratic<f32> for Quad {
    fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c, r1: 0.0, r2: 0.0 }
    }

    fn quadratic_eq(&mut self) {
        self.r1 = -self.b + (((self.b * self.b) - 4.0 * self.a * self.c)
                               .sqrt()) / (2.0 * self.a);    
        self.r2 = -self.b - (((self.b * self.b) - 4.0 * self.a * self.c)
                               .sqrt()) / (2.0 * self.a);
    }
    
    fn get_roots(&self) -> (f32, f32) {
        (self.r1, self.r2)
    }
    
    fn fill_quads(nvar: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut quads = Vec::with_capacity(N);
        for _ in 0..nvar {
            let a = rng.gen_range(-MAX_VALUE..=MAX_VALUE);
            let c = match a.is_sign_positive() {
                true => rng.gen_range(0.0..=MAX_VALUE),
                false => rng.gen_range(-MAX_VALUE..=0.0),
            };
            let b = match rng.gen_bool(0.5) {
                true => rng.gen_range((4.0_f32 * a * c).sqrt()..=(MAX_VALUE * 2.0)),
                false => rng.gen_range(-(MAX_VALUE * 2.0)..=-(4.0_f32 * a * c).sqrt()),
            };
            quads.push(Quad::new(a, b, c));
        }
        quads
    }
}

fn main() {
    use std::time::Instant;
    let mut quadscom = ComplexQuad::fill_quads(N);
    let quadscom2 = quadscom.clone();
    let mut quadsfloat = Quad::fill_quads(N);
    let quadsfloat2 = quadsfloat.clone();
    let now = Instant::now();
    {
        quadscom.par_iter_mut().for_each(|q| q.quadratic_eq());
    }
    let elapsed = now.elapsed();
    let now2 = Instant::now();
    {
        quadsfloat.par_iter_mut().for_each(|q| q.quadratic_eq());
        // quadsfloat.iter().for_each(|q| println!("{:?}", q.get_roots()));
    }
    let elapsed2 = now2.elapsed();
    let elapsed4 = now4.elapsed();
    println!("\n Multithreaded:");
    println!("Complex root calculation time elapsed: {:.2?}", elapsed);
    println!("Float root calculation time elapsed: {:.2?}", elapsed2);
}
