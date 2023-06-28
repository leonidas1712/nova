use std::time::{Duration, Instant};

pub fn measure<F: FnOnce() -> ()>(function: F) -> Duration {
    let start = Instant::now();
    function();
    start.elapsed()
}
