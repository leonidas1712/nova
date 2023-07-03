use crate::{evaluate_input_tco};
use crate::evaluator::context_tco::EvalContext;
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
    let expr = "(recr 10000)";

    // let expr="(recr_t 1000 0)";
    // (def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))
    let mut ctx = EvalContext::new();
    let res = evaluate_input_tco(recr, &mut ctx);
    println!("Function: {}", res);

    let mut total: f64 = 0.0;

    for i in 0..n + 1 {
        let start = Instant::now();
        let r2 = evaluate_input_tco(expr, &mut ctx);
        let end = start.elapsed();
        total += end.as_secs_f64();

        println!("{}, time: {}", r2, end.as_secs_f64());
    }

    println!("Avg: {}", total / (f64::from(n)));
}

pub fn time_comp(n: u32) {
    // let recr="(def recr_t (n acc) (if (eq n 0) acc (recr_t (pred n) (add acc n))))";

    let recr = "(def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))";
    let mut ctx = EvalContext::new();
    let res = evaluate_input_tco(recr, &mut ctx);
    println!("Defined function:{}", res);

    let mut i=2;
    let mut count=0.0;

    let mut past:f64=1.0;
    let mut total_ratio:f64=1.0;

    loop {
        if i > n {
            break;
        }

        let expr = format!("(recr {})", i);
        println!("Expr:{}", expr);

        let start=Instant::now();
        let res=evaluate_input_tco(expr.as_str(),&mut ctx);
        let end=start.elapsed();

        println!("Result:{}", res);
        println!("Time taken:{}", end.as_secs_f64());

        let ratio=(end.as_secs_f64())/(past);

        println!("Ratio:{}",ratio);
        println!("");
        
        if count>0.0 {
            total_ratio=total_ratio+ratio;
        } else {
            total_ratio=total_ratio+0.0;
        }

        past=end.as_secs_f64();

        i*=2;
        count=count+1.0;
    }

    let avg=total_ratio/(count);

    println!("");
    println!("Count (for ratios summed):{}", count);
    println!("Avg ratio:{}", avg);

    // let expr="(recr_t 1000 0)";
    // (def recr (n) (if (eq n 0) 0 (add n (recr (pred n)))))
 
  
    // println!("Avg: {}", total / (f64::from(n)));
}
