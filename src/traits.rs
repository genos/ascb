/// Core algebraic traits

/// A set with a closed associative binary operation
///
/// `PartialEq` because we use equality in property tests
pub trait Semigroup: PartialEq {
    fn op(x: &Self, y: &Self) -> Self;
}

/// A semigroup with an identity element (here named zero)
///
/// Ideally we'd like to have this be a constant, but some instances require this to be a function
/// (e.g. HashMap & String).
pub trait Monoid: Semigroup {
    fn zero() -> Self;
}

/// A monoid whose operation is commutative
pub trait CommutativeMonoid: Monoid {}

/// A commutative monoid with an additional operation and identity element (one)
pub trait Semiring: CommutativeMonoid {
    fn one() -> Self;
    fn mul(x: &Self, y: &Self) -> Self;
}

/// Simultaneously map items to a monoid and accumulate them
pub fn fold_map<T, I, M, F>(xs: I, f: F) -> M
where
    I: Iterator<Item = T>,
    M: Monoid,
    F: Fn(T) -> M,
{
    xs.fold(M::zero(), |m, t| M::op(&m, &f(t)))
}

/// This pops up _lots_ of places.
pub fn power_semigroup<S: Semigroup + Clone>(x: S, n: u64) -> S {
    assert!(
        n >= 1,
        "Semigroups don't necessarily have an identity element."
    );
    let mut y = x.clone();
    let mut m = n;
    while m > 1 {
        if m & 1 == 1 {
            y = S::op(&y, &x);
        }
        y = S::op(&y, &y);
        m >>= 1;
    }
    y
}

/// Monoid version, accepting 0 as an argument.
pub fn power_monoid<M: Monoid + Clone>(x: M, n: u64) -> M {
    if n == 0 {
        M::zero()
    } else {
        power_semigroup(x, n)
    }
}
