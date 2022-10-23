use crate::traits::SuperSized;
use std::mem::size_of;

macro_rules! impl_for_primitive {
    ($ty:ty) => {
        impl SuperSized for $ty {
            const SIZE: usize = size_of::<$ty>();
        }
    };
}

impl_for_primitive!(u8);
impl_for_primitive!(u16);
impl_for_primitive!(u32);
impl_for_primitive!(u64);
impl_for_primitive!(u128);
impl_for_primitive!(i8);
impl_for_primitive!(i16);
impl_for_primitive!(i32);
impl_for_primitive!(i64);
impl_for_primitive!(i128);
impl_for_primitive!(f32);
impl_for_primitive!(f64);
impl_for_primitive!(usize);
impl_for_primitive!(isize);
impl_for_primitive!(bool);
impl_for_primitive!(());

macro_rules! impl_for_primitive_arr {
    ($ty:ty) => {
        impl<const N: usize> SuperSized for [$ty; N] {
            const SIZE: usize = N * <$ty>::SIZE;
        }
    };
}

impl_for_primitive_arr!(u8);
impl_for_primitive_arr!(u16);
impl_for_primitive_arr!(u32);
impl_for_primitive_arr!(u64);
impl_for_primitive_arr!(u128);
impl_for_primitive_arr!(i8);
impl_for_primitive_arr!(i16);
impl_for_primitive_arr!(i32);
impl_for_primitive_arr!(i64);
impl_for_primitive_arr!(i128);
impl_for_primitive_arr!(f32);
impl_for_primitive_arr!(f64);
impl_for_primitive_arr!(usize);
impl_for_primitive_arr!(isize);
impl_for_primitive_arr!(bool);
impl_for_primitive_arr!(());

impl<T: SuperSized> SuperSized for Option<T> {
    const SIZE: usize = 1 + T::SIZE;
}

impl<A: SuperSized> SuperSized for (A,) {
    const SIZE: usize = A::SIZE;
}
impl<A: SuperSized, B: SuperSized> SuperSized for (A, B) {
    const SIZE: usize = A::SIZE + B::SIZE;
}
impl<A: SuperSized, B: SuperSized, C: SuperSized> SuperSized for (A, B, C) {
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE;
}
impl<A: SuperSized, B: SuperSized, C: SuperSized, D: SuperSized> SuperSized for (A, B, C, D) {
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE + D::SIZE;
}
impl<A: SuperSized, B: SuperSized, C: SuperSized, D: SuperSized, E: SuperSized> SuperSized
    for (A, B, C, D, E)
{
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE + D::SIZE + E::SIZE;
}
impl<A: SuperSized, B: SuperSized, C: SuperSized, D: SuperSized, E: SuperSized, F: SuperSized>
    SuperSized for (A, B, C, D, E, F)
{
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE + D::SIZE + E::SIZE + F::SIZE;
}
impl<
        A: SuperSized,
        B: SuperSized,
        C: SuperSized,
        D: SuperSized,
        E: SuperSized,
        F: SuperSized,
        G: SuperSized,
    > SuperSized for (A, B, C, D, E, F, G)
{
    const SIZE: usize = A::SIZE + B::SIZE + C::SIZE + D::SIZE + E::SIZE + F::SIZE + G::SIZE;
}
impl<
        A: SuperSized,
        B: SuperSized,
        C: SuperSized,
        D: SuperSized,
        E: SuperSized,
        F: SuperSized,
        G: SuperSized,
        H: SuperSized,
    > SuperSized for (A, B, C, D, E, F, G, H)
{
    const SIZE: usize =
        A::SIZE + B::SIZE + C::SIZE + D::SIZE + E::SIZE + F::SIZE + G::SIZE + H::SIZE;
}
