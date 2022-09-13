//! Macros for generating property tests for the required properties

#[macro_export]
macro_rules! semigroup_properties {
    ($arb:expr) => {
        proptest! {
            #[test]
            fn associativity(x in $arb(), y in $arb(), z in $arb()) {
                prop_assert_eq!(
                    Semigroup::op(&x, &Semigroup::op(&y, &z)),
                    Semigroup::op(&Semigroup::op(&x, &y), &z)
                );
            }
        }
    };
}

#[macro_export]
macro_rules! monoid_properties {
    ($arb:expr) => {
        mod semigroup_properties {
            use super::*;
            semigroup_properties!($arb);
        }
        proptest! {
            #[test]
            fn left_zero(x in $arb()) {
                prop_assert_eq!(Semigroup::op(&Monoid::zero(), &x), x);
            }
            #[test]
            fn right_zero(x in $arb()) {
                prop_assert_eq!(Semigroup::op(&x, &Monoid::zero()), x);
            }
        }
    };
}

#[macro_export]
macro_rules! commutative_monoid_properties {
    ($arb:expr) => {
        mod monoid_properties {
            use super::*;
            monoid_properties!($arb);
        }
        proptest! {
            #[test]
            fn commutativity(x in $arb(), y in $arb()) {
                prop_assert_eq!(Semigroup::op(&x, &y), Semigroup::op(&y, &x));
            }
        }
    };
}

#[macro_export]
macro_rules! semiring_properties {
    ($arb: expr) => {
        mod commutative_monoid_properties {
            use super::*;
            commutative_monoid_properties!($arb);
        }
        proptest! {
            #[test]
            fn left_annihilation(x in $arb()) {
                prop_assert_eq!(Semiring::mul(&Monoid::zero(), &x), Monoid::zero());
            }
            #[test]
            fn right_annihilation(x in $arb()) {
                prop_assert_eq!(Semiring::mul(&x, &Monoid::zero()), Monoid::zero());
            }
            #[test]
            fn left_one(x in $arb()) {
                prop_assert_eq!(Semiring::mul(&Semiring::one(), &x), x);
            }
            #[test]
            fn right_one(x in $arb()) {
                prop_assert_eq!(Semiring::mul(&x, &Semiring::one()), x);
            }
            #[test]
            fn associativity(x in $arb(), y in $arb(), z in $arb()) {
                prop_assert_eq!(
                    Semiring::mul(&x, &Semiring::mul(&y, &z)),
                    Semiring::mul(&Semiring::mul(&x, &y), &z)
                );
            }
            #[test]
            fn left_distribution(x in $arb(), y in $arb(), z in $arb()) {
                prop_assert_eq!(Semiring::mul(&x, &Semigroup::op(&y, &z)),
                                Semigroup::op(&Semiring::mul(&x, &y), &Semiring::mul(&x, &z))
                );
            }
            #[test]
            fn right_distribution(x in $arb(), y in $arb(), z in $arb()) {
                prop_assert_eq!(Semiring::mul(&Semigroup::op(&x, &y), &z),
                                Semigroup::op(&Semiring::mul(&x, &z), &Semiring::mul(&y, &z))
                );
            }
        }
    };
}
