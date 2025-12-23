//! Core algebraic traits
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    num::NonZeroU64,
};

/// A set with a closed associative binary operation
pub trait Semigroup {
    /// Associative operation
    fn op(x: &Self, y: &Self) -> Self;
}

/// A semigroup with an identity element (here named zero)
///
/// Ideally we'd like to have this be a constant, but some instances require this to be a function
/// (e.g. `HashMap` & String).
pub trait Monoid: Semigroup {
    /// Identity element for [`Semigroup::op`]
    fn zero() -> Self;
}

/// A monoid whose operation is commutative
pub trait CommutativeMonoid: Monoid {}

/// A commutative monoid with an additional operation and identity element (one)
pub trait Semiring: CommutativeMonoid {
    /// Additional associative binary operation
    fn mul(x: &Self, y: &Self) -> Self;
    /// Identity element for [`Semiring::mul`]
    fn one() -> Self;
}

/// Simultaneously map items to a monoid and accumulate them
pub fn fold_map<T, M: Monoid>(xs: impl Iterator<Item = T>, f: impl Fn(T) -> M) -> M {
    xs.fold(M::zero(), |m, t| M::op(&m, &f(t)))
}

/// This pops up _lots_ of places.
pub fn power_semigroup<S: Semigroup + Clone>(x: &S, n: NonZeroU64) -> S {
    let mut y = x.clone();
    let mut m = n.get();
    while m > 1 {
        if m & 1 == 1 {
            y = S::op(&y, x);
        }
        y = S::op(&y, &y);
        m >>= 1;
    }
    y
}

/// Monoid version, accepting 0 as an argument
pub fn power_monoid<M: Monoid + Clone>(x: &M, n: u64) -> M {
    NonZeroU64::new(n).map_or_else(M::zero, |p| power_semigroup(x, p))
}

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
impl<K: Clone + Eq + Hash, V: Semigroup + Clone, S: BuildHasher + Default> Semigroup
    for HashMap<K, V, S>
{
    fn op(x: &Self, y: &Self) -> Self {
        let mut h = HashMap::default();
        for (k, v) in x.iter().chain(y.iter()) {
            h.entry(k.clone())
                .and_modify(|w| *w = V::op(w, v))
                .or_insert_with(|| v.clone());
        }
        h
    }
}

/// A map of {key ↦ value} is a monoid if the values form a semigroup.
impl<K: Clone + Eq + Hash, V: Semigroup + Clone, S: BuildHasher + Default> Monoid
    for HashMap<K, V, S>
{
    fn zero() -> Self {
        HashMap::default()
    }
}
