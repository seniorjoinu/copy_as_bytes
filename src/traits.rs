pub trait SuperSized: Sized {
    const SIZE: usize;

    #[inline]
    fn super_size() -> usize {
        Self::SIZE
    }

    #[inline]
    fn super_size_u8_arr() -> [u8; Self::SIZE] {
        [0u8; Self::SIZE]
    }
}

pub trait AsBytes: SuperSized {
    fn to_bytes(self) -> [u8; Self::SIZE];

    fn from_bytes(arr: [u8; Self::SIZE]) -> Self;
}
