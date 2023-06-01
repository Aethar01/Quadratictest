pub mod quads;
use rayon::prelude::*;
use quads::{ComplexQuad, F32Quad, I32Quad, Quadratic};

const N: usize = 100;

fn timeit<F: FnMut()>(mut f: F) -> std::time::Duration {
    let now = std::time::Instant::now();
    f();
    now.elapsed()
}

fn main() {
    let mut quadscom = ComplexQuad::fill_quads(N);
    let mut quadsfloat = F32Quad::fill_quads(N);
    let mut quadsint = I32Quad::fill_quads(N);

    let multicomplexelapsed = timeit(|| {
        quadscom.par_iter_mut().for_each(|q| q.quadratic_eq());
    });

    let multifloatelapsed = timeit(|| {
        quadsfloat.par_iter_mut().for_each(|q| q.quadratic_eq());
    });

    let multiintelapsed = timeit(|| {
        quadsint.par_iter_mut().for_each(|q| q.quadratic_eq());
    });

    let singlecomplexelapsed = timeit(|| {
        quadscom.clone().into_iter().for_each(|mut q| q.quadratic_eq());
    });

    let singlefloatelapsed = timeit(|| {
        quadsfloat.clone().into_iter().for_each(|mut q| q.quadratic_eq());
    });

    let singleintelapsed = timeit(|| {
        quadsint.clone().into_iter().for_each(|mut q| q.quadratic_eq());
    });
    
    // let now = Instant::now();
    // {
    //     quadscom.par_iter_mut().for_each(|q| q.quadratic_eq());
    // }
    // let elapsed = now.elapsed();
    // let now2 = Instant::now();
    // {
    //     quadsfloat.par_iter_mut().for_each(|q| q.quadratic_eq());
    //     // quadsfloat.iter().for_each(|q| println!("{:?}", q.get_roots()));
    // }
    // let elapsed2 = now2.elapsed();
    // let now3 = Instant::now();
    // {
    //     quadscom2.into_iter().for_each(|mut q| q.quadratic_eq());
    // }
    // let elapsed3 = now3.elapsed();
    // let now4 = Instant::now();
    // {
    //     quadsfloat2.into_iter().for_each(|mut q| q.quadratic_eq());
    // }
    // let elapsed4 = now4.elapsed();
    println!("\n Multithreaded:");
    println!("Complex root calculation time elapsed: {:.2?}", multicomplexelapsed);
    println!("Float root calculation time elapsed: {:.2?}", multifloatelapsed);
    println!("Int root calculation time elapsed: {:.2?}", multiintelapsed);
    println!("\n Singlethreaded:");
    println!("Complex root calculation time elapsed: {:.2?}", singlecomplexelapsed);
    println!("Float root calculation time elapsed: {:.2?}", singlefloatelapsed);
    println!("Int root calculation time elapsed: {:.2?}", singleintelapsed);
}
