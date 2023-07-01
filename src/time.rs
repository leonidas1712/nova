use crate::evaluate_input;
use crate::setup_context;
use std::time::{Duration, Instant};

pub fn measure<F: FnOnce() -> ()>(function: F) -> Duration {
    let start = Instant::now();
    function();
    start.elapsed()
}

// (recr_t 1000 0)

pub fn bench(n: u32) {
    // let recr="(def recr_t (n acc) (if (eq n 0) acc (recr_t (pred n) (add acc n))))";

    let recr = "(def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))";
    let expr = "(recr 1000)";

    // let expr="(recr_t 1000 0)";
    // (def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))
    let mut ctx = setup_context();
    let res = evaluate_input(recr, &mut ctx);
    println!("Function: {}", res);

    let mut total: f64 = 0.0;

    for i in 0..n + 1 {
        let start = Instant::now();
        let r2 = evaluate_input(expr, &mut ctx);
        let end = start.elapsed();
        total += end.as_secs_f64();

        println!("{}, time: {}", r2, end.as_secs_f64());
    }

    println!("Avg: {}", total / (f64::from(n)));
}
