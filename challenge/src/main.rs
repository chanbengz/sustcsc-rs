mod lwe;
mod solver;

use lwe::generate_lwe_instance;
use ndarray::Array1;
use std::time::Instant;

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

fn main() {
    let test_cases = vec![
        (10, 100, 97, 0.005),
        (20, 400, 193, 0.005),
        (30, 900, 389, 0.005),
        (40, 1500, 769, 0.005),
        (45, 1700, 12289, 0.005),
        (50, 2500, 1543, 0.005),
        (55, 3600, 6151, 0.005),
        (30, 1000, 3079, 0.010),
        (40, 1500, 6151, 0.010),
    ];

    for (i, &(n, m, q, alpha)) in test_cases.iter().enumerate() {
        println!("----------------------------------------");
        println!(
            "Test Case {}: n={}, m={}, q={}, alpha={}",
            i + 1,
            n,
            m,
            q,
            alpha
        );
        let instance = generate_lwe_instance(n, m, q, alpha);

        let start = Instant::now();
        let s_pred = solver::solve_lwe(n, m, q, alpha, &instance.a, &instance.b);
        let duration = start.elapsed();

        let error_norm = compute_error_norm(&instance.s, &s_pred, q);

        println!("Error Norm: {:.4}", error_norm);
        println!("Execution Time: {:.4} s", duration.as_secs_f64());
        println!("----------------------------------------");
    }
}
