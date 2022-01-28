use std::time::Duration;

/// if use cogo runtime
#[cfg(feature = "cogo")]
pub type Receiver<T> = cogo::std::sync::channel::Receiver<T>;
#[cfg(feature = "cogo")]
pub type Sender<T> = cogo::std::sync::channel::Sender<T>;

#[cfg(feature = "cogo")]
pub fn chan<T>() -> (Sender<T>, Receiver<T>) {
    cogo::chan!()
}

#[cfg(feature = "cogo")]
pub fn sleep(d: Duration) {
    cogo::coroutine::sleep(d)
}

#[cfg(feature = "cogo")]
pub fn spawn<F>(f: F) where F: FnOnce() + std::marker::Send + 'static {
    cogo::go!(cogo::coroutine::Builder::new().stack_size(2*0x1000),f);
}

/// if not cogo
#[cfg(not(feature = "cogo"))]
pub type Receiver<T> = crossbeam::channel::Receiver<T>;
#[cfg(not(feature = "cogo"))]
pub type Sender<T> = crossbeam::channel::Sender<T>;

#[cfg(not(feature = "cogo"))]
pub fn chan<T>() -> (Sender<T>, Receiver<T>) {
    crossbeam::channel::unbounded()
}

#[cfg(not(feature = "cogo"))]
pub fn sleep(d: Duration) {
    std::thread::sleep(d)
}

#[cfg(not(feature = "cogo"))]
pub fn spawn<F>(f: F) where F: FnOnce() + std::marker::Send + 'static {
    std::thread::spawn(f);
}