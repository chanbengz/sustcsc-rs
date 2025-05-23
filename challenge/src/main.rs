mod lwe;
mod solver;

use std::time::Instant;
use lwe::generate_lwe_instance;
use ndarray::Array1;

fn compute_error_norm(s_true: &Array1<u64>, s_pred: &Array1<u64>, q: u64) -> f64 {
    s_true.iter()
        .zip(s_pred.iter())
        .map(|(&a, &b)| {
            let diff = (a as i64 - b as i64).rem_euclid(q as i64);
            let min_diff = diff.min(q as i64 - diff);
            (min_diff * min_diff) as f64
        })
        .sum::<f64>()
        .sqrt()
}

fn main() {
    let n = 50;
    let m = 400;
    let q = 3329;
    let alpha = 0.005;

    let instance = generate_lwe_instance(n, m, q, alpha);

    let start = Instant::now();
    let s_pred = solver::solve_lwe(instance.n, instance.m, instance.q, instance.alpha, &instance.A, &instance.b);
    let duration = start.elapsed();

    let error_norm = compute_error_norm(&instance.s, &s_pred, instance.q);

    println!("Error Norm: {:.4}", error_norm);
    println!("Execution Time: {:.4} seconds", duration.as_secs_f64());
}
