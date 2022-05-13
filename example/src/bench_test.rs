use fast_log::appender::{FastLogFormatRecord, FastLogRecord, LogAppender};
use fast_log::bencher::QPS;
use fast_log::config::Config;
use fast_log::filter::NoFilter;
use fast_log::sleep;
use std::time::{Duration, Instant};

struct BenchRecvLog {}

impl LogAppender for BenchRecvLog {
    fn do_log(&self, record: &FastLogRecord) {
        //do nothing
    }
}

/// cargo run --release --package example --bin bench_test
fn main() {
    fast_log::init(Config::new().custom(BenchRecvLog {}));
    let total = 1000000;
    let now = Instant::now();
    for index in 0..total {
        log::info!("Commencing yak shaving{}", index);
    }
    now.time(total);
    now.qps(total);
    log::logger().flush();
}
