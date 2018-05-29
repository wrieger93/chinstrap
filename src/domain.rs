use std::collections::BTreeSet;
use std::iter::FromIterator;

use traits::{Bounded, Domain};

pub struct BTreeSetDomain<T> {
    values: BTreeSet<T>,
}

impl<T> FromIterator<T> for BTreeSetDomain<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> BTreeSetDomain<T>
    where
        I: IntoIterator<Item = T>,
    {
        BTreeSetDomain {
            values: BTreeSet::from_iter(iter),
        }
    }
}

impl<T> Domain for BTreeSetDomain<T>
where
    T: Clone + Ord,
{
    type Value = T;

    fn size(&self) -> usize {
        self.values.len()
    }

    fn contains(&self, value: &Self::Value) -> bool {
        self.values.contains(value)
    }

    fn assign(&mut self, value: &Self::Value) -> Result<(), ()> {
        if self.values.len() == 0 {
            Ok(())
        } else {
            if let Some(taken_value) = self.values.take(value) {
                self.values.clear();
                self.values.insert(taken_value);
                Ok(())
            } else {
                self.values.clear();
                Err(())
            }
        }
    }

    fn remove(&mut self, value: &Self::Value) -> Result<(), ()> {
        if self.values.len() == 0 {
            Ok(())
        } else {
            self.values.remove(value);
            if self.values.len() == 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }
}

impl<T> Bounded for BTreeSetDomain<T>
where
    T: Clone + Ord,
{
    fn min(&self) -> Option<&Self::Value> {
        self.values.range(..).next()
    }

    fn max(&self) -> Option<&Self::Value> {
        self.values.range(..).next_back()
    }

    fn adjust_min(&mut self, new_min: &Self::Value) -> Result<(), ()> {
        if self.values.len() == 0 {
            Ok(())
        } else {
            self.values = self.values.split_off(new_min);
            if self.values.len() == 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    fn adjust_max(&mut self, new_max: &Self::Value) -> Result<(), ()> {
        if self.values.len() == 0 {
            Ok(())
        } else {
            let mut others = self.values.split_off(new_max);
            if let Some(max) = others.take(new_max) {
                self.values.insert(max);
            }
            if self.values.len() == 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }
}
