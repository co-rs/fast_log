
#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    use crossbeam_utils::sync::WaitGroup;
    use log::{debug, info, Level,error};

    use fast_log::appender::{FastLogFormatRecord, FastLogRecord, LogAppender};
    use fast_log::bencher::QPS;
    use fast_log::consts::LogSize;
    use fast_log::filter::NoFilter;
    use fast_log::plugin::file_split::{RollingType};
    use fast_log::{init_custom_log, init_log, init_split_log};
    use fast_log::plugin::packer::{ZipPacker, LZ4Packer};



    #[test]
    pub fn test_log() {
        let wait = init_log("requests.log", 1000, log::Level::Debug, None, true).unwrap();
        debug!("Commencing yak shaving{}", 0);
        wait.wait();
    }

    //cargo test --release --color=always --package fast_log --lib fast_log::test::bench_log --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    pub fn bench_log() {
        init_log("requests.log", 10000, log::Level::Info, None, false);
        let total = 100000;
        let now = Instant::now();
        for index in 0..total {
            //sleep(Duration::from_secs(1));
            info!("Commencing yak shaving{}", index);
        }
        now.time(total);
        now.qps(total);
        sleep(Duration::from_secs(1));
    }

    struct CustomLog {}

    impl LogAppender for CustomLog {
        fn do_log(&self, records: &mut [FastLogRecord]) {
            for record in records {
                let data;
                match record.level {
                    Level::Warn | Level::Error => {
                        data = format!(
                            "{} {} {} - {}  {}\n",
                            &record.now,
                            record.level,
                            record.module_path,
                            record.args,
                            record.format_line()
                        );
                    }
                    _ => {
                        data = format!(
                            "{} {} {} - {}\n",
                            &record.now, record.level, record.module_path, record.args
                        );
                    }
                }
                print!("{}", data);
            }
        }
    }

    #[test]
    pub fn test_custom() {
        init_custom_log(
            vec![Box::new(CustomLog {})],
            1000,
            log::Level::Info,
            Box::new(NoFilter {}),
            Box::new(FastLogFormatRecord {}),
        );
        info!("Commencing yak shaving");
        error!("Commencing error");
        sleep(Duration::from_secs(1));
    }

    #[test]
    pub fn test_file_compation() {
        init_split_log(
            "target/logs/",
            1000,
            LogSize::MB(1),
            false,
            RollingType::All,
            log::Level::Info,
            None,
            Box::new(ZipPacker{}),
            true,
        );
        for _ in 0..20000 {
            info!("Commencing yak shaving");
        }
        sleep(Duration::from_secs(1));
    }

    #[test]
    pub fn test_file_compation_zip() {
        init_split_log(
            "target/logs/",
            1000,
            LogSize::KB(50),
            true,
            RollingType::KeepNum(5),
            log::Level::Info,
            None,
            Box::new(ZipPacker{}),
            true,
        );
        for _ in 0..20000 {
            info!("Commencing yak shaving");
        }
        sleep(Duration::from_secs(10));
    }

    #[test]
    pub fn test_file_compation_zip_stable_test() {
        init_split_log(
            "target/logs/",
            1000,
            LogSize::MB(100),
            true,
            RollingType::All,
            log::Level::Info,
            None,
            Box::new(LZ4Packer{}),
            false,
        );
        let now = std::time::Instant::now();
        loop {
            info!("Commencing yak shaving");
            if now.elapsed() > Duration::from_secs(30) {
                break;
            }
        }
        info!("done >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>{:?}",now.elapsed());
        sleep(Duration::from_secs(10));
    }

    #[test]
    pub fn test_wait() {
        let wg = WaitGroup::new();
        let wg1 = wg.clone();
        std::thread::spawn(move || {
            // Do some work.
            // Drop the reference to the wait group.
            drop(wg1);
        });
        wg.wait()
    }

    #[test]
    pub fn test_wait_log_exit() {
        let wait_group = init_log("requests.log", 1000, log::Level::Info, None, false).unwrap();
        for index in 0..10000 {
            info!("index:{}", index);
        }
        let now = std::time::Instant::now();
        wait_group.wait();
        println!("wait:{:?}", now.elapsed());
    }

    struct BenchRecvLog {}

    impl LogAppender for BenchRecvLog {
        fn do_log(&self, record: &mut [FastLogRecord]) {}
    }

    //cargo test --release --package fast_log --lib example::fast_log_test::test::bench_recv --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    pub fn bench_recv() {
        init_custom_log(
            vec![Box::new(BenchRecvLog {})],
            1000,
            log::Level::Info,
            Box::new(NoFilter {}),
            Box::new(FastLogFormatRecord {}),
        );
        let total = 10000;
        let now = Instant::now();
        for index in 0..total {
            //sleep(Duration::from_secs(1));
            info!("Commencing yak shaving{}", index);
        }
        now.time(total);
        now.qps(total);
        sleep(Duration::from_secs(1));
    }
}

fn main(){

}