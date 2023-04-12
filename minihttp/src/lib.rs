pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(count: usize) -> Self {
        ThreadPool {}
    }

    pub fn execute<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
// where
//     F: FnOnce() -> T,
//     F: Send + 'static,
//     T: Send + 'static,
// {
//     Builder::new().spawn(f).expect("failed to spawn thread")
// }
