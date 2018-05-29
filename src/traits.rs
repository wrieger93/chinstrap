use std::iter::FromIterator;

pub trait Domain: FromIterator<<Self as Domain>::Value> {
    type Value: Clone;

    fn size(&self) -> usize;
    fn contains(&self, value: &Self::Value) -> bool;
    fn assign(&mut self, value: &Self::Value) -> Result<(), ()>;
    fn remove(&mut self, value: &Self::Value) -> Result<(), ()>;
}

pub trait Bounded: Domain
where
    <Self as Domain>::Value: Ord,
{
    fn min(&self) -> Option<&Self::Value>;
    fn max(&self) -> Option<&Self::Value>;
    fn adjust_min(&mut self, new_min: &Self::Value) -> Result<(), ()>;
    fn adjust_max(&mut self, new_max: &Self::Value) -> Result<(), ()>;
}
