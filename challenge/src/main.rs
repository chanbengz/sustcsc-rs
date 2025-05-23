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
    let test_cases = vec![
        (10, 30, 97, 0.01),
        (20, 60, 193, 0.02),
        (30, 90, 389, 0.03),
        (40, 120, 769, 0.04),
        (50, 150, 1543, 0.05),
        (60, 180, 3079, 0.06),
        (70, 210, 6151, 0.07),
        (80, 240, 12289, 0.08),
        (90, 270, 24593, 0.09),
        (100, 300, 49157, 0.10),
    ];

    for (i, &(n, m, q, alpha)) in test_cases.iter().enumerate() {
        println!("----------------------------------------");
        println!("Test Case {}: n={}, m={}, q={}, alpha={}", i + 1, n, m, q, alpha);
        let instance = generate_lwe_instance(n, m, q, alpha);

        let start = Instant::now();
        let s_pred = solver::solve_lwe(n, m, q, alpha, &instance.a, &instance.b);
        let duration = start.elapsed();

        let error_norm = compute_error_norm(&instance.s, &s_pred, q);

        println!("Error Norm: {:.4}", error_norm);
        println!("Execution Time: {:.4} ms", duration.as_millis());
        println!("----------------------------------------");
    }
}
