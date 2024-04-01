use crate::appender::{LogAppender, RecordFormat};
use crate::consts::LogSize;
use crate::filter::{Filter};
use crate::plugin::console::ConsoleAppender;
use crate::plugin::file::FileAppender;
use crate::plugin::file_loop::FileLoopAppender;
use crate::plugin::file_rotate::{FileRotateAppender, Rotate};
use crate::plugin::file_split::{FileSplitAppender, Keep, Packer, RawFile, SplitFile};
use crate::FastLogFormat;
use dark_std::sync::SyncVec;
use log::LevelFilter;
use std::fmt::{Debug, Formatter};
use parking_lot::Mutex;

/// the fast_log Config
/// for example:
/// ```rust
/// use fast_log::Config;
/// fn main(){
///    fast_log::init(Config::new().console().chan_len(Some(1000000))).unwrap();
/// }
/// ```
pub struct Config {
    /// Each appender is responsible for printing its own business
    /// every LogAppender have one thread(need Mutex) access this.
    pub appends: SyncVec<Mutex<Box<dyn LogAppender>>>,
    /// the log level filter
    pub level: LevelFilter,
    /// filter log
    pub filters: SyncVec<Box<dyn Filter>>,
    /// format record into field fast_log_record's formatted:String
    pub format: Box<dyn RecordFormat>,
    /// the channel length,default None(Unbounded channel)
    pub chan_len: Option<usize>,
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("appends", &self.appends.len())
            .field("level", &self.level)
            .field("chan_len", &self.chan_len)
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            appends: SyncVec::new(),
            level: LevelFilter::Trace,
            filters: SyncVec::new(),
            format: Box::new(FastLogFormat::new()),
            chan_len: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// set log LevelFilter
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }
    /// add log Filter
    pub fn add_filter<F: Filter + 'static>(self, filter: F) -> Self {
        self.filters.push(Box::new(filter));
        self
    }

    /// add log Filter
    pub fn filter(self, filters: Vec<Box<dyn Filter>>) -> Self {
        for x in filters {
            self.filters.push(x);
        }
        self
    }
    /// set log format
    pub fn format<F: RecordFormat + 'static>(mut self, format: F) -> Self {
        self.format = Box::new(format);
        self
    }
    /// add a ConsoleAppender
    pub fn console(self) -> Self {
        self.appends.push(Mutex::new(Box::new(ConsoleAppender {})));
        self
    }
    /// add a FileAppender
    pub fn file(self, file: &str) -> Self {
        self.appends
            .push(Mutex::new(Box::new(FileAppender::new(file).unwrap())));
        self
    }
    /// add a FileLoopAppender
    pub fn file_loop(self, file: &str, max_temp_size: LogSize) -> Self {
        self.appends.push(Mutex::new(Box::new(
            FileLoopAppender::<RawFile>::new(file, max_temp_size).expect("make file_loop fail"),
        )));
        self
    }
    /// add a FileSplitAppender
    pub fn file_split<P: Packer + Sync + 'static, R: Keep + 'static>(
        self,
        file_path: &str,
        temp_size: LogSize,
        rolling_type: R,
        packer: P,
    ) -> Self {
        self.appends.push(Mutex::new(Box::new(
            FileSplitAppender::<RawFile>::new(file_path, temp_size, rolling_type, Box::new(packer))
                .unwrap(),
        )));
        self
    }

    /// add a FileRotateAppender
    pub fn file_rotate<P: Packer + Sync + 'static, R: Keep + Rotate + 'static>(
        self,
        file_path: &str,
        temp_size: LogSize,
        rolling_type: R,
        packer: P,
    ) -> Self {
        self.appends.push(Mutex::new(Box::new(
            FileRotateAppender::<RawFile, R>::new(
                file_path,
                temp_size,
                rolling_type,
                Box::new(packer),
            )
            .unwrap(),
        )));
        self
    }

    /// add a SplitAppender
    /// .split::<FileType, Packer>()
    /// for example:
    ///
    // fast_log::init(
    //         Config::new()
    //             .chan_len(Some(100000))
    //             .split::<MmapFile, LogPacker>(
    //                 "target/logs/temp.log",
    //                 LogSize::MB(1),
    //                 RollingType::All,
    //                 LogPacker {},
    //             ),
    //     );
    pub fn split<F: SplitFile + 'static, R: Keep + 'static, P: Packer + Sync + 'static>(
        self,
        file_path: &str,
        temp_size: LogSize,
        keeper: R,
        packer: P,
    ) -> Self {
        self.appends.push(Mutex::new(Box::new(
            FileSplitAppender::<F>::new(file_path, temp_size, keeper, Box::new(packer)).unwrap(),
        )));
        self
    }
    /// add a custom LogAppender
    pub fn custom<Appender: LogAppender + 'static>(self, arg: Appender) -> Self {
        self.add_appender(arg)
    }

    /// add a LogAppender
    pub fn add_appender<Appender: LogAppender + 'static>(self, arg: Appender) -> Self {
        self.appends.push(Mutex::new(Box::new(arg)));
        self
    }

    /// if none=> unbounded() channel,if Some =>  bounded(len) channel
    pub fn chan_len(mut self, len: Option<usize>) -> Self {
        self.chan_len = len;
        self
    }
}
