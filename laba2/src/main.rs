extern crate blas_src;

use num_complex::Complex64;
use rand::RngExt;
use rayon::prelude::*;
use std::hint::black_box;
use std::time::Instant;

const N: usize = 4096;

fn calculate_complexity(n: usize) -> f64 {
    2.0 * (n as f64).powi(3)
}

fn calculate_mflops(complexity: f64, time_seconds: f64) -> f64 {
    (complexity / time_seconds) * 1e-6
}

fn mult_native(a: &[Complex64], b: &[Complex64], c: &mut [Complex64]) {
    for i in 0..N {
        for j in 0..N {
            let mut sum = Complex64::new(0.0, 0.0);
            for k in 0..N {
                sum += a[i * N + k] * b[k * N + j];
            }
            c[i * N + j] = sum;
        }
    }
}

fn mult_blas(a: &[Complex64], b: &[Complex64], c: &mut [Complex64]) {
    unsafe {
        let alpha = Complex64::new(1.0, 0.0);
        let beta = Complex64::new(0.0, 0.0);

        cblas_sys::cblas_zgemm(
            cblas_sys::CblasRowMajor,
            cblas_sys::CblasNoTrans,
            cblas_sys::CblasNoTrans,
            N as i32,
            N as i32,
            N as i32,
            &alpha as *const _ as *const _,
            a.as_ptr() as *const _,
            N as i32,
            b.as_ptr() as *const _,
            N as i32,
            &beta as *const _ as *const _,
            c.as_mut_ptr() as *mut _,
            N as i32,
        );
    }
}

const BLOCK_SIZE: usize = 512;

fn mult_rayon_optimized(a: &[Complex64], b: &[Complex64], c: &mut [Complex64]) {
    c.par_chunks_mut(BLOCK_SIZE * N)
        .enumerate()
        .for_each(|(panel_idx, c_panel)| {
            let i_start = panel_idx * BLOCK_SIZE;
            let panel_height = c_panel.len() / N;

            for j_block in (0..N).step_by(BLOCK_SIZE) {
                let j_end = (j_block + BLOCK_SIZE).min(N);

                for k_block in (0..N).step_by(BLOCK_SIZE) {
                    let k_end = (k_block + BLOCK_SIZE).min(N);

                    for i_local in 0..panel_height {
                        let i = i_start + i_local;

                        let c_row = &mut c_panel[(i_local * N + j_block)..(i_local * N + j_end)];

                        for k in k_block..k_end {
                            let a_ik = a[i * N + k];
                            let b_row = &b[(k * N + j_block)..(k * N + j_end)];

                            for (c_val, &b_val) in c_row.iter_mut().zip(b_row.iter()) {
                                *c_val += a_ik * b_val;
                            }
                        }
                    }
                }
            }
        });
}

fn main() {
    println!("Морозов К.О. 090301-ПОВа-о25");

    let mut rng = rand::rng();

    let mut a = vec![Complex64::new(0.0, 0.0); N * N];
    let mut b = vec![Complex64::new(0.0, 0.0); N * N];

    for i in 0..(N * N) {
        a[i] = Complex64::new(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0));
        b[i] = Complex64::new(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0));
    }

    let complexity = calculate_complexity(N);
    println!("Вычислительная сложность: {:.2e}", complexity);

    let mut c1 = vec![Complex64::new(0.0, 0.0); N * N];
    let mut c2 = vec![Complex64::new(0.0, 0.0); N * N];
    let mut c3 = vec![Complex64::new(0.0, 0.0); N * N];

    println!("\n[2] BLAS (cblas_zgemm)...");
    let start_blas = Instant::now();
    mult_blas(&a, &b, &mut c2);
    let time_blas = start_blas.elapsed().as_secs_f64();
    println!(
        "t: {:.4} с | res: {:.2} MFlops",
        time_blas,
        calculate_mflops(complexity, time_blas)
    );

    println!("\n[3] Rayon...");
    let start_rayon = Instant::now();
    mult_rayon_optimized(&a, &b, &mut c3);
    let time_rayon = start_rayon.elapsed().as_secs_f64();
    println!(
        "t: {:.4} с | res: {:.2} MFlops",
        time_rayon,
        calculate_mflops(complexity, time_rayon)
    );

    println!("\n[1] Bad algo...");
    let start_naive = Instant::now();
    mult_native(&a, &b, &mut c1);
    black_box(&c1); // Иначе 4581298449066.67 MFlops =)
    let time_naive = start_naive.elapsed().as_secs_f64();
    println!(
        "t: {:.4} с | res: {:.2} MFlops",
        time_naive,
        calculate_mflops(complexity, time_naive)
    );
}
