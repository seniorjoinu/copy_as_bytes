pub trait SuperSized: Sized {
    const SIZE: usize;

    fn super_size() -> usize {
        Self::SIZE
    }
}

pub trait AsBytes: SuperSized {
    fn to_bytes(self) -> [u8; Self::SIZE];

    fn from_bytes(arr: [u8; Self::SIZE]) -> Self;
}
