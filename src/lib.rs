
#[derive(Default)]
#[must_use]
pub struct Parallel<'a, T> {
    closures: Vec<Box<dyn FnOnce() -> T + Send + 'a>>
}

impl<'a, T> Parallel<'a, T> {

    pub fn new() -> Parallel<'a, T> {
        Parallel {
            closures: Vec::new()
        }
    }

    pub fn add<F>(mut self, f: F) -> Parallel<'a, T>
    where
        F: FnOnce() -> T + Send + 'a,
        T: Send + 'a
    {
        self.closures.push(Box::new(f));
        self
    }
}