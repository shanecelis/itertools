use std::fmt;
use std::iter::FusedIterator;
use std::usize;
use alloc::vec::Vec;

use super::combinations::{Combinations, checked_binomial, combinations};

/// An iterator to iterate through the powerset of the elements from an iterator.
///
/// See [`.powerset()`](crate::Itertools::powerset) for more
/// information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Powerset<I: Iterator> {
    combs: Combinations<I>,
}

impl<I> Clone for Powerset<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(combs);
}

impl<I> fmt::Debug for Powerset<I>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(Powerset, combs);
}

/// Create a new `Powerset` from a clonable iterator.
pub fn powerset<I>(src: I) -> Powerset<I>
    where I: Iterator,
          I::Item: Clone,
{
    Powerset {
        combs: combinations(src, 0),
    }
}

impl<I> Iterator for Powerset<I>
    where
        I: Iterator,
        I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(elt) = self.combs.next() {
            Some(elt)
        } else if self.combs.k() < self.combs.n()
            || self.combs.k() == 0
        {
            self.combs.reset(self.combs.k() + 1);
            self.combs.next()
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let k = self.combs.k();
        // Total bounds for source iterator.
        let (n_min, n_max) = self.combs.src().size_hint();
        // Total bounds for the current combinations.
        let (mut low, mut upp) = self.combs.size_hint();
        low = remaining_for(low, n_min, k).unwrap_or(usize::MAX);
        upp = upp.and_then(|upp| remaining_for(upp, n_max?, k));
        (low, upp)
    }

    fn count(self) -> usize {
        let k = self.combs.k();
        let (n, combs_count) = self.combs.n_and_count();
        remaining_for(combs_count, n, k).unwrap()
    }
}

impl<I> FusedIterator for Powerset<I>
    where
        I: Iterator,
        I::Item: Clone,
{}

fn remaining_for(init_count: usize, n: usize, k: usize) -> Option<usize> {
    (k + 1..=n).fold(Some(init_count), |sum, i| {
        sum.and_then(|s| s.checked_add(checked_binomial(n, i)?))
    })
}
