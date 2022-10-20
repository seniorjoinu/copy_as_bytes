use crate::traits::{AsBytes, SuperSized};
use std::mem::size_of;

impl AsBytes for u8 {
    #[inline]
    fn to_bytes(self) -> [u8; u8::SIZE] {
        [self]
    }

    #[inline]
    fn from_bytes(arr: [u8; u8::SIZE]) -> Self {
        arr[0]
    }
}

impl AsBytes for i8 {
    #[inline]
    fn to_bytes(self) -> [u8; i8::SIZE] {
        [self as u8]
    }

    #[inline]
    fn from_bytes(arr: [u8; i8::SIZE]) -> Self {
        arr[0] as i8
    }
}

impl AsBytes for bool {
    #[inline]
    fn to_bytes(self) -> [u8; bool::SIZE] {
        [u8::from(self)]
    }

    #[inline]
    fn from_bytes(arr: [u8; bool::SIZE]) -> Self {
        debug_assert!(arr[0] < 2);

        arr[0] == 1
    }
}

macro_rules! impl_for_numbers {
    ($ty:ty) => {
        impl AsBytes for $ty {
            #[inline]
            fn to_bytes(self) -> [u8; <$ty>::SIZE] {
                self.to_le_bytes()
            }

            #[inline]
            fn from_bytes(arr: [u8; <$ty>::SIZE]) -> Self {
                Self::from_le_bytes(arr)
            }
        }
    };
}

impl_for_numbers!(u16);
impl_for_numbers!(u32);
impl_for_numbers!(u64);
impl_for_numbers!(u128);
impl_for_numbers!(i16);
impl_for_numbers!(i32);
impl_for_numbers!(i64);
impl_for_numbers!(i128);
impl_for_numbers!(f32);
impl_for_numbers!(f64);
impl_for_numbers!(usize);
impl_for_numbers!(isize);

impl AsBytes for () {
    #[inline]
    fn to_bytes(self) -> [u8; size_of::<Self>()] {
        []
    }

    #[inline]
    fn from_bytes(_: [u8; size_of::<Self>()]) -> Self {}
}

impl AsBytes for [u8; 128] {
    #[inline]
    fn to_bytes(self) -> [u8; Self::SIZE] {
        self
    }

    #[inline]
    fn from_bytes(arr: [u8; Self::SIZE]) -> Self {
        arr
    }
}
