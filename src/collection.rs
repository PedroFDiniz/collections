pub trait Collection<T> {
    fn add(&mut self, value: T);
    fn remove(&mut self, value: T) -> bool where T: PartialEq;
    fn contains(&self, value: T) -> bool where T: PartialEq;
    fn size(&self) -> usize;
}