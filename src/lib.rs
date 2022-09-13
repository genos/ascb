//! Mostly interesting for the property-based tests.
pub mod gaussian;
pub mod instances;
pub mod traits;

pub use crate::gaussian::Gaussian;
pub use crate::traits::*;

#[cfg(test)]
mod properties;

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::identities::Zero;
    use num_traits::real::Real;
    use proptest::prelude::*;
    use std::ops::{Add, Mul};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Max(f64);
    impl Semigroup for Max {
        fn op(&Max(x): &Self, &Max(y): &Self) -> Self {
            Max(x.max(y))
        }
    }
    impl Monoid for Max {
        fn zero() -> Self {
            Max(f64::NEG_INFINITY)
        }
    }
    impl CommutativeMonoid for Max {}

    mod max {
        use super::*;
        use prop::collection::vec;
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use rayon::prelude::*;
        commutative_monoid_properties!(|| any::<f64>().prop_map(Max));
        proptest! {
            #[test]
            fn map_shuffle_reduce_sort_of(xs in vec(any::<f64>(), 0..1000)) {
                let map_reduce = xs.iter().cloned().map(Max).fold(Monoid::zero(), |x, y| Semigroup::op(&x, &y));
                let map_shuffle_reduce = {
                    let mut ys = xs.into_iter().map(Max).collect::<Vec<_>>();
                    ys.shuffle(&mut thread_rng());
                    ys
                }.par_chunks(4)
                    .map(|c| c.into_iter().fold(Monoid::zero(), |x, &y| Semigroup::op(&x, &y)))
                    .reduce(Monoid::zero, |x, y| Semigroup::op(&x, &y));
                prop_assert_eq!(map_reduce, map_shuffle_reduce);
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Any(bool);
    impl Semigroup for Any {
        fn op(&Any(x): &Self, &Any(y): &Self) -> Self {
            Any(x || y)
        }
    }
    impl Monoid for Any {
        fn zero() -> Self {
            Any(false)
        }
    }

    #[derive(Debug, PartialEq)]
    struct All(bool);
    impl Semigroup for All {
        fn op(&All(x): &Self, &All(y): &Self) -> Self {
            All(x && y)
        }
    }
    impl Monoid for All {
        fn zero() -> Self {
            All(true)
        }
    }

    mod any {
        use super::*;
        monoid_properties!(|| any::<bool>().prop_map(Any));
    }

    mod all {
        use super::*;
        monoid_properties!(|| any::<bool>().prop_map(All));
    }

    #[derive(Debug, PartialEq)]
    struct TaggedU64<const ADDING: bool>(u64); // Boolean blindness :-(
    impl Semigroup for TaggedU64<true> {
        fn op(&TaggedU64(x): &Self, &TaggedU64(y): &Self) -> Self {
            TaggedU64(x.wrapping_add(y))
        }
    }
    impl Semigroup for TaggedU64<false> {
        fn op(&TaggedU64(x): &Self, &TaggedU64(y): &Self) -> Self {
            TaggedU64(x.wrapping_mul(y))
        }
    }

    impl Monoid for TaggedU64<true> {
        fn zero() -> Self {
            TaggedU64(0)
        }
    }
    impl Monoid for TaggedU64<false> {
        fn zero() -> Self {
            TaggedU64(1)
        }
    }

    mod u64_add {
        use super::*;
        monoid_properties!(|| any::<u64>().prop_map(TaggedU64::<true>));
    }

    mod u64_mul {
        use super::*;
        monoid_properties!(|| any::<u64>().prop_map(TaggedU64::<false>));
    }

    impl Semigroup for String {
        fn op(x: &Self, y: &Self) -> Self {
            format!("{}{}", x, y)
        }
    }
    impl Monoid for String {
        fn zero() -> Self {
            "".to_string()
        }
    }

    mod string {
        use super::*;
        monoid_properties!(|| any::<String>());
    }

    impl<T: PartialEq + Copy> Semigroup for Vec<T> {
        fn op(xs: &Self, ys: &Self) -> Self {
            let mut zs = Vec::with_capacity(xs.len() + ys.len());
            for z in xs.iter().chain(ys.iter()) {
                zs.push(*z);
            }
            zs
        }
    }
    impl<T: PartialEq + Copy> Monoid for Vec<T> {
        fn zero() -> Self {
            Vec::new()
        }
    }

    mod vec {
        use super::*;
        monoid_properties!(any::<Vec<u8>>);
    }

    mod gaussian {
        use super::*;
        use prop::collection::vec;
        use rayon::prelude::*;
        use std::iter::FromIterator;
        monoid_properties!(|| vec(-1e3..1e3, 0..1000).prop_map(Gaussian::from_iter));
        proptest! {
            #[test]
            fn homomorphisms_and_associativity_are_cool(xs in vec(-1e3..1e3, 0..1000)) {
                let from_iter = xs.iter().collect();
                let by_hand = xs.iter().fold(Gaussian::default(), |g, &x| g + x);
                let mapped = xs.iter().map(|&x| Gaussian::new(x)).fold(Monoid::zero(), |g1, g2| Semigroup::op(&g1, &g2));
                let mapped_chunks = xs
                    .chunks(4)
                    .map(|c| c.iter().map(|&x| Gaussian::new(x)).fold(Monoid::zero(), |g1, g2| Semigroup::op(&g1, &g2)))
                    .fold(Monoid::zero(), |g1, g2| Semigroup::op(&g1, &g2));
                let mapped_par_chunks = xs
                    .par_chunks(4)
                    .map(|c| c.iter().map(|&x| Gaussian::new(x)).fold(Monoid::zero(), |g1, g2| Semigroup::op(&g1, &g2)))
                    .reduce(Monoid::zero, |g1, g2| Semigroup::op(&g1, &g2));
                let sharper_par = xs
                    .par_chunks(4)
                    .map(|c| {let mut g = Gaussian::default(); c.iter().for_each(|&x| g += x); g})
                    .reduce(Monoid::zero, |g1, g2| Semigroup::op(&g1, &g2));
                let par_from_iter = xs
                    .par_chunks(4)
                    .map(|c| c.iter().collect())
                    .reduce(Monoid::zero, |g1, g2| Semigroup::op(&g1, &g2));
                for w in [from_iter, by_hand, mapped, mapped_chunks, mapped_par_chunks, sharper_par, par_from_iter].windows(2) {
                    prop_assert_eq!(w[0], w[1]);
                }
            }
        }
    }

    mod tuples {
        use super::*;
        monoid_properties!(|| any::<(f64, bool)>().prop_map(|(x, b)| (Max(x), Any(b))));
    }

    mod options {
        use super::*;
        monoid_properties!(|| any::<Option<f64>>().prop_map(|o| o.map(Max)));
    }

    mod hashmap {
        use super::*;
        use prop::collection::hash_map as hm;
        fn max_float() -> impl Strategy<Value = Max> {
            any::<f64>().prop_map(Max)
        }
        monoid_properties!(|| hm(any::<char>(), max_float(), 0..100));
        mod nested {
            use super::*;
            monoid_properties!(|| hm(
                any::<char>(),
                hm(any::<char>(), hm(0u8..255u8, max_float(), 3), 0..3),
                0..3
            ));
        }
        mod composite {
            use super::*;
            fn pairs() -> impl Strategy<Value = (Max, Any)> {
                any::<(f64, bool)>().prop_map(|(x, b)| (Max(x), Any(b)))
            }
            monoid_properties!(|| hm(
                any::<char>(),
                hm(any::<char>(), hm(0u8..255u8, pairs(), 3), 0..3),
                0..3
            ));
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum MinPlus<T: Real> {
        Infinity,
        Finite(T),
    }
    impl<T: Real> Default for MinPlus<T> {
        fn default() -> Self {
            MinPlus::Infinity
        }
    }
    impl<T: Real> Add for MinPlus<T> {
        type Output = Self;
        fn add(self, y: Self) -> Self::Output {
            match (self, y) {
                (MinPlus::Infinity, _) => y,
                (_, MinPlus::Infinity) => self,
                (MinPlus::Finite(a), MinPlus::Finite(b)) => MinPlus::Finite(a.min(b)),
            }
        }
    }
    impl<T: Real> Mul for MinPlus<T> {
        type Output = Self;
        fn mul(self, y: Self) -> Self::Output {
            match (self, y) {
                (MinPlus::Infinity, _) => MinPlus::Infinity,
                (_, MinPlus::Infinity) => MinPlus::Infinity,
                (MinPlus::Finite(a), MinPlus::Finite(b)) => MinPlus::Finite(a + b),
            }
        }
    }
    impl<T: Real> Semigroup for MinPlus<T> {
        fn op(&x: &Self, &y: &Self) -> Self {
            x + y
        }
    }
    impl<T: Real> Monoid for MinPlus<T> {
        fn zero() -> Self {
            MinPlus::Infinity
        }
    }
    impl<T: Real> CommutativeMonoid for MinPlus<T> {}
    impl<T: Real> Semiring for MinPlus<T> {
        fn one() -> Self {
            MinPlus::Finite(Zero::zero())
        }
        fn mul(&x: &Self, &y: &Self) -> Self {
            x * y
        }
    }

    mod minplus {
        use super::*;
        semiring_properties!(|| any::<Option<f64>>().prop_map(|o| {
            match o {
                None => MinPlus::Infinity,
                Some(x) => MinPlus::Finite(x),
            }
        }));
    }
}
