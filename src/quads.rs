use num::complex::Complex;
use rand::Rng;

const N: usize = 100;
const MAX_VALUE: f32 = 100.0;

#[derive(Debug, Copy, Clone)]
pub struct ComplexQuad {
    a: Complex<f32>,
    b: Complex<f32>,
    c: Complex<f32>,
    r1: Complex<f32>,
    r2: Complex<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct F32Quad {
    a: f32,
    b: f32,
    c: f32,
    r1: f32,
    r2: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct I32Quad {
    a: i32,
    b: i32,
    c: i32,
    r1: f32,
    r2: f32,
}

pub trait Quadratic<T> {
    fn new(a: T, b: T, c: T) -> Self;
    fn quadratic_eq(&mut self);
    fn get_roots(&self) -> (T, T);
    fn fill_quads(nvar: usize) -> Vec<Self> where Self: Sized;
}

impl Quadratic<i32> for I32Quad {
    fn new(a: i32, b: i32, c: i32) -> Self {
        Self { a, b, c, r1: 0.0, r2: 0.0 }
    }

    fn quadratic_eq(&mut self) {
        self.r1 = -self.b as f32 + (((self.b as f32 * self.b as f32) - 4.0 * self.a as f32 * self.c as f32)
                               .sqrt() as f32) / (2 * self.a) as f32;    
        self.r2 = -self.b as f32 - (((self.b as f32 * self.b as f32) - 4.0 * self.a as f32 * self.c as f32)
                               .sqrt() as f32) / (2 * self.a) as f32;
    }

    fn get_roots(&self) -> (i32, i32) {
        (self.r1 as i32, self.r2 as i32)
    }

    fn fill_quads(nvar: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut quads = Vec::with_capacity(N);
        for _ in 0..nvar {
            let a = rng.gen_range(-MAX_VALUE as i32..=MAX_VALUE as i32);
            let c = match a.is_positive() {
                true => rng.gen_range(0..=MAX_VALUE as i32),
                false => rng.gen_range(-MAX_VALUE as i32..=0),
            };
            let b = if 4 * a * c < 0 {
                rng.gen_range(-MAX_VALUE as i32..=MAX_VALUE as i32)
            } else {
                match rng.gen_bool(0.5) {
                    true => rng.gen_range((( 4 * a * c ) as f32).sqrt() as i32..=(MAX_VALUE * 2.0) as i32),
                    false => rng.gen_range(-(MAX_VALUE * 2.0) as i32..=-(( 4 * a * c ) as f32).sqrt() as i32),
                }
                // rng.gen_range((4.0_f32 * a * c).sqrt()..=(MAX_VALUE * 2.0))
            }; 
            quads.push(I32Quad::new(a, b, c));
        }
        quads
    }
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

impl Quadratic<f32> for F32Quad {
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
            let b = if 4.0_f32 * a * c < 0.0 {
                rng.gen_range(-MAX_VALUE..=MAX_VALUE)
            } else {
                match rng.gen_bool(0.5) {
                    true => rng.gen_range((4.0_f32 * a * c).sqrt()..=(MAX_VALUE * 2.0)),
                    false => rng.gen_range(-(MAX_VALUE * 2.0)..=-(4.0_f32 * a * c).sqrt()),
                }
                // rng.gen_range((4.0_f32 * a * c).sqrt()..=(MAX_VALUE * 2.0))
            }; 
            quads.push(F32Quad::new(a, b, c));
        }
        quads
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_i32_quad() {
        let mut quad = I32Quad::new(4, 4, 1);
        quad.quadratic_eq();
        println!("{:?}", quad);
        assert_eq!(quad.get_roots(), (-1, 0));
    }

    #[test]
    fn test_f32_quad() {
        let mut quad = F32Quad::new(4.0, 4.0, 1.0);
        quad.quadratic_eq();
        println!("{:?}", quad);
        assert_eq!(quad.get_roots(), (-0.5, 0.0));
    }

    #[test]
    fn test_complex_quad() {
        let mut quad = ComplexQuad::new(Complex::new(4.0, 0.0), Complex::new(4.0, 0.0), Complex::new(1.0, 0.0));
        quad.quadratic_eq();
        println!("{:?}", quad);
        assert_eq!(quad.get_roots(), (Complex::new(-0.5, 0.0), Complex::new(0.0, 0.0)));
    }
}
