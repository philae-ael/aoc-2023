pub mod cli;

pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl<I, T> ToVec<I> for T
where
    T: Iterator<Item = I>,
{
    fn to_vec(self) -> Vec<I> {
        self.collect()
    }
}

