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

macro_rules! impl_for_u8_arr {
    ($size:expr) => {
        impl AsBytes for [u8; $size] {
            #[inline]
            fn to_bytes(self) -> [u8; Self::SIZE] {
                self
            }

            #[inline]
            fn from_bytes(arr: [u8; Self::SIZE]) -> Self {
                arr
            }
        }
    };
}

impl_for_u8_arr!(0);
impl_for_u8_arr!(1);
impl_for_u8_arr!(2);
impl_for_u8_arr!(4);
impl_for_u8_arr!(8);
impl_for_u8_arr!(16);
impl_for_u8_arr!(29); // for principals
impl_for_u8_arr!(32);
impl_for_u8_arr!(64);
impl_for_u8_arr!(128);
impl_for_u8_arr!(256);
impl_for_u8_arr!(512);
impl_for_u8_arr!(1024);
impl_for_u8_arr!(2048);
impl_for_u8_arr!(4096);

impl<T: AsBytes> AsBytes for Option<T>
    where [(); T::SIZE]: Sized,
{
    fn to_bytes(self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        if let Some(it) = self {
            buf[0] = 1;
            buf[1..].copy_from_slice(&it.to_bytes());
        }

        buf
    }

    fn from_bytes(arr: [u8; Self::SIZE]) -> Self {
        if arr[0] == 0 {
            None
        } else {
            let mut buf = [0u8; T::SIZE];
            buf.copy_from_slice(&arr[1..]);
            
            Some(T::from_bytes(buf))
        }
    }
}
