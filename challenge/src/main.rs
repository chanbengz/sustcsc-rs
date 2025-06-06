#![allow(unused_variables, non_snake_case)]
mod lwe;
mod solver;

use crate::lwe::generate_lwe_instance;
use ndarray::Array1;
use std::time::Instant;

fn main() {
    let test_cases = vec![
        (10, 100,  97,    0.005),
        (20, 400,  193,   0.005),
        (30, 900,  389,   0.005),
        (40, 1500, 769,   0.005),
        (45, 1700, 12289, 0.005),
        (50, 2500, 1543,  0.005),
        (55, 3600, 6151,  0.005),
        (45, 1700, 3079,  0.010),
        (40, 1500, 6151,  0.015),
    ];

    println!(" case     n      m       q   alpha   error norm   time (s)");
    println!("----- ----- ------ ------- ------- ------------ ----------");
    for (i, &(n, m, q, alpha)) in test_cases.iter().enumerate() {
        let instance = generate_lwe_instance(n, m, q, alpha);
        let start = Instant::now();
        let s_pred = solver::solve_lwe(n, m, q, alpha, &instance.a, &instance.b);
        let duration = start.elapsed();
        let error_norm = compute_error_norm(&instance.s, &s_pred, q);

        println!("{:5} {:5} {:6} {:7} {:7.3} {:12.4} {:10.4}", 
            i, n, m, q, alpha, 
            error_norm,
            duration.as_secs_f64()
        );
    }
}

fn compute_error_norm(s_true: &Array1<u64>, s_pred: &Array1<u64>, q: u64) -> f64 {
    s_true
        .iter()
        .zip(s_pred.iter())
        .map(|(&a, &b)| {
            let diff = (a as i64 - b as i64).rem_euclid(q as i64);
            let min_diff = diff.min(q as i64 - diff);
            (min_diff * min_diff) as f64
        })
        .sum::<f64>()
        .sqrt()
}