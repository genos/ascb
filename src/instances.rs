//! Some useful standalone instances
use crate::traits::*;
use std::collections::HashMap;
use std::hash::Hash;

/// The direct product of two semigroups is a semigroup.
impl<X: Semigroup, Y: Semigroup> Semigroup for (X, Y) {
    fn op((a, x): &Self, (b, y): &Self) -> Self {
        (X::op(a, b), Y::op(x, y))
    }
}

/// The direct product of two monoids is a monoid.
impl<X: Monoid, Y: Monoid> Monoid for (X, Y) {
    fn zero() -> Self {
        (X::zero(), Y::zero())
    }
}

/// A Semigroup can be made into a monoid by adjoining a new identity element.
impl<T: Semigroup + Clone> Semigroup for Option<T> {
    fn op(x: &Self, y: &Self) -> Self {
        match (x, y) {
            (Some(a), Some(b)) => Some(T::op(a, b)),
            (None, _) => y.clone(),
            (_, None) => x.clone(),
        }
    }
}

/// A Semigroup can be made into a monoid by adjoining a new identity element.
impl<T: Semigroup + Clone> Monoid for Option<T> {
    fn zero() -> Self {
        None
    }
}

/// A map of {key ↦ value} is a semigroup if the values form one.
impl<K: Clone + Eq + Hash, V: Semigroup + Clone> Semigroup for HashMap<K, V> {
    fn op(x: &Self, y: &Self) -> Self {
        let mut h = HashMap::new();
        for (k, v) in x.iter().chain(y.iter()) {
            h.entry((*k).clone())
                .and_modify(|w| *w = V::op(w, v))
                .or_insert_with(|| v.clone());
        }
        h
    }
}

/// A map of {key ↦ value} is a monoid if the values form a semigroup.
impl<K: Clone + Eq + Hash, V: Semigroup + Clone> Monoid for HashMap<K, V> {
    fn zero() -> Self {
        HashMap::new()
    }
}
