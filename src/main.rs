pub mod quads;
use rayon::prelude::*;
use quads::{ComplexQuad, F32Quad, I32Quad, Quadratic};

const N: usize = 100;

fn main() {
    use std::time::Instant;
    let mut quadscom = ComplexQuad::fill_quads(N);
    let quadscom2 = quadscom.clone();
    let mut quadsfloat = F32Quad::fill_quads(N);
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
    let now3 = Instant::now();
    {
        quadscom2.into_iter().for_each(|mut q| q.quadratic_eq());
    }
    let elapsed3 = now3.elapsed();
    let now4 = Instant::now();
    {
        quadsfloat2.into_iter().for_each(|mut q| q.quadratic_eq());
    }
    let elapsed4 = now4.elapsed();
    println!("\n Multithreaded:");
    println!("Complex root calculation time elapsed: {:.2?}", elapsed);
    println!("Float root calculation time elapsed: {:.2?}", elapsed2);
    println!("\n Singlethreaded:");
    println!("Complex root calculation time elapsed: {:.2?}", elapsed3);
    println!("Float root calculation time elapsed: {:.2?}", elapsed4);
}
